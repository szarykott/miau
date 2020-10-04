use crate::{
    configuration::{CompoundKey, Key, TypedValue},
    error::{ConfigurationError, ErrorCode},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::{TryFrom, TryInto}, fmt::Display, default::Default};

#[derive(Debug)]
pub struct Configuration {
    roots : Vec<ConfigurationRoot>
}

impl Configuration {
    pub fn get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        for root in self.roots.iter().rev() {
            if let v @ Some(_) = root.get::<T>(keys) {
                return v
            }
        }

        None
    }

    pub fn add_root(&mut self, root : ConfigurationRoot) {
        self.roots.push(root);
    }
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            roots : Vec::new()
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigurationRoot {
    Value(Option<TypedValue>),
    Map(HashMap<String, ConfigurationRoot>),
    Array(Vec<ConfigurationRoot>),
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

impl ConfigurationRoot {
    fn node_type(&self) -> NodeType {
        match self {
            ConfigurationRoot::Value(_) => NodeType::Value,
            ConfigurationRoot::Map(_) => NodeType::Map,
            ConfigurationRoot::Array(_) => NodeType::Array,
        }
    }

    pub fn get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        let mut node = Result::Ok(self);
        for key in keys {
            node = node.and_then(|nd| nd.descend(key));
        }
        node.ok()
            .and_then(|node| node.get_value::<T>().unwrap_or_default())
    }

    // TODO: Those errors are not really needed, it can be Option
    fn descend(&self, key: &Key) -> Result<&ConfigurationRoot, ConfigurationError> {
        match self {
            ConfigurationRoot::Value(_) => match key {
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
            ConfigurationRoot::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(key.clone(), *index).into()),
                },
                Key::Map(inner_key) => {
                    Err(ErrorCode::WrongKeyType(key.clone(), inner_key.to_owned()).into())
                }
            },
            ConfigurationRoot::Map(map) => match key {
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
            ConfigurationRoot::Value(Some(v)) => match T::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            ConfigurationRoot::Value(None) => Ok(None),
            ConfigurationRoot::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Array).into())
            }
            ConfigurationRoot::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Map).into())
            }
        }
    }

    // MERGING //

    pub fn merge(
        previous: ConfigurationRoot,
        next: ConfigurationRoot,
    ) -> Result<ConfigurationRoot, ConfigurationError> {
        match (previous, next) {
            (_, vn @ ConfigurationRoot::Value(_)) => Ok(vn),
            (ConfigurationRoot::Map(mp), ConfigurationRoot::Map(mn)) => {
                Ok(ConfigurationRoot::Map(ConfigurationRoot::merge_maps(mp, mn)?))
            }
            (ConfigurationRoot::Array(_), nn @ ConfigurationRoot::Array(_)) => Ok(nn),
            (vp, vm) => {
                Err(
                    ErrorCode::IncompatibleNodeSubstitution(None, vp.node_type(), vm.node_type())
                        .into(),
                )
            }
        }
    }

    fn merge_maps(
        mut previous: HashMap<String, ConfigurationRoot>,
        mut next: HashMap<String, ConfigurationRoot>,
    ) -> Result<HashMap<String, ConfigurationRoot>, ConfigurationError> {
        for (key, next_node) in next.drain() {
            if !previous.contains_key(&key) {
                previous.insert(key, next_node);
            } else {
                let previous_node = previous.remove(&key).unwrap();
                match (previous_node, next_node) {
                    (ConfigurationRoot::Value(_), vn @ ConfigurationRoot::Value(_)) => {
                        previous.insert(key, vn);
                    }
                    (ConfigurationRoot::Map(mp), ConfigurationRoot::Map(mn)) => {
                        previous.insert(key, ConfigurationRoot::Map(ConfigurationRoot::merge_maps(mp, mn)?));
                    }
                    (ConfigurationRoot::Array(mut vp), ConfigurationRoot::Array(mut vn)) => {
                        if vp.len() > vn.len() {
                            for (index, root) in vn.drain(..).enumerate() {
                                vp[index] = root;
                            }
                            previous.insert(key, ConfigurationRoot::Array(vp));
                        } else {
                            previous.insert(key, ConfigurationRoot::Array(vn));
                        }
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

macro_rules! try_from_for {
    ($($t:ty),*) => {
        $(impl TryFrom<&ConfigurationRoot> for $t {
            type Error = ConfigurationError;
        
            fn try_from(value: &ConfigurationRoot) -> Result<Self, Self::Error> {
                match value {
                    ConfigurationRoot::Value(Some(tv)) => tv.try_into(),
                    ConfigurationRoot::Value(None) => Err(ErrorCode::MissingValue.into()),
                    ConfigurationRoot::Map(_) => Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Map).into()),
                    ConfigurationRoot::Array(_) => Err(ErrorCode::UnexpectedNodeType(None, NodeType::Value, NodeType::Array).into()),
                }
            }
        })*
    };
}

try_from_for!(i8, i16, i32, i64, isize, f32, f64, bool, String);