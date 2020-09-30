use crate::{
    configuration::{CompoundKey, Key, TypedValue},
    error::{ConfigurationError, ErrorCode},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Configuration {
    Value(Option<TypedValue>),
    Map(HashMap<String, Configuration>),
    Array(Vec<Configuration>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeType {
    Value,
    Map,
    Array,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Value => write!(f, "Value"),
            NodeType::Map => write!(f, "Map"),
            NodeType::Array => write!(f, "Array"),
        }
    }
}

impl Configuration {
    pub fn drill_get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        let mut node = Result::Ok(self);
        for key in keys {
            node = node.and_then(|nd| nd.get(key));
        }
        node.ok()
            .and_then(|node| node.get_value::<T>().unwrap_or_default())
    }

    fn node_type(&self) -> NodeType {
        match self {
            Configuration::Value(_) => NodeType::Value,
            Configuration::Map(_) => NodeType::Map,
            Configuration::Array(_) => NodeType::Array,
        }
    }

    fn get(&self, key: &Key) -> Result<&Configuration, ConfigurationError> {
        match self {
            Configuration::Value(_) => match key {
                Key::Array(_) => Err(ErrorCode::UnexpectedNodeType(
                    Some(key.clone()),
                    NodeType::Array,
                    NodeType::Value,
                )
                .into()),
                Key::Map(_) => Err(ErrorCode::UnexpectedNodeType(
                    Some(key.clone()),
                    NodeType::Map,
                    NodeType::Value,
                )
                .into()),
            },
            Configuration::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(key.clone(), *index).into()),
                },
                Key::Map(inner_key) => {
                    Err(ErrorCode::WrongKeyType(key.clone(), inner_key.to_owned()).into())
                }
            },
            Configuration::Map(map) => match key {
                Key::Array(i) => Err(ErrorCode::WrongKeyType(key.clone(), i.to_string()).into()),
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::KeyNotFound(key.clone(), k.to_owned()).into()), //TODO: Fix this error
                },
            },
        }
    }

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        match self {
            Configuration::Value(Some(v)) => match T::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            Configuration::Value(None) => Ok(None),
            Configuration::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Array).into())
            }
            Configuration::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Map).into())
            }
        }
    }

    pub fn merge(
        previous: Configuration,
        next: Configuration,
    ) -> Result<Configuration, ConfigurationError> {
        match (previous, next) {
            (Configuration::Value(vp), Configuration::Value(vn)) => {
                if TypedValue::is_substitutable_type_option(&vp, &vn) {
                    Ok(Configuration::Value(vn))
                } else {
                    Err(ErrorCode::IncompatibleValueSubstitution(
                        None,
                        TypedValue::display_option(vp),
                        TypedValue::display_option(vn),
                    )
                    .into())
                }
            }
            (Configuration::Map(mp), Configuration::Map(mn)) => {
                Ok(Configuration::Map(Configuration::merge_maps(mp, mn)?))
            }
            (Configuration::Array(_), nn @ Configuration::Array(_)) => Ok(nn),
            (vp, vm) => {
                Err(
                    ErrorCode::IncompatibleNodeSubstitution(None, vp.node_type(), vm.node_type())
                        .into(),
                )
            }
        }
    }

    fn merge_maps(
        mut previous: HashMap<String, Configuration>,
        mut next: HashMap<String, Configuration>,
    ) -> Result<HashMap<String, Configuration>, ConfigurationError> {
        for (key, next_node) in next.drain() {
            if !previous.contains_key(&key) {
                previous.insert(key, next_node);
            } else {
                let previous_node = previous.remove(&key).unwrap();
                match (previous_node, next_node) {
                    (Configuration::Value(v1), Configuration::Value(v2)) => {
                        if TypedValue::is_substitutable_type_option(&v1, &v2) {
                            previous.insert(key, Configuration::Value(v2));
                        } else {
                            return Err(ErrorCode::IncompatibleValueSubstitution(
                                None,
                                TypedValue::display_option(v1),
                                TypedValue::display_option(v2),
                            )
                            .into());
                        }
                    }
                    (Configuration::Map(mp), Configuration::Map(mn)) => {
                        let merged = Configuration::merge_maps(mp, mn)?;
                        previous.insert(key, Configuration::Map(merged));
                    }
                    (Configuration::Array(_), nn @ Configuration::Array(_)) => {
                        previous.insert(key, nn);
                    }
                    (vp, vn) => {
                        return Err(ErrorCode::IncompatibleNodeSubstitution(
                            None,
                            vp.node_type(),
                            vn.node_type(),
                        )
                        .into())
                    }
                }
            }
        }

        Ok(previous)
    }
}
