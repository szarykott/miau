use super::{CompoundKey, Key, TypedValue};
use crate::error::{ConfigurationAccessError, ConfigurationMergeError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Configuration {
    Value(Option<TypedValue>),
    Map(HashMap<String, Configuration>),
    Array(Vec<Configuration>),
}

#[derive(Debug)]
pub enum NodeType {
    Value,
    Map,
    Array
}

impl Configuration {
    pub fn drill_get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationAccessError>,
    {
        let mut node = Result::Ok(self);
        for key in keys {
            node = node.and_then(|nd| nd.get(key));
        }
        node.ok()
            .and_then(|node| node.get_value::<T>().unwrap_or_default())
    }

    fn get(&self, key: &Key) -> Result<&Configuration, ConfigurationAccessError> {
        match self {
            Configuration::Value(_) => Err(ConfigurationAccessError::UnexpectedNodeType(
                "not value",
                "value",
            )),
            Configuration::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ConfigurationAccessError::IndexOutOfRange(*index)),
                },
                Key::Map(key) => Err(ConfigurationAccessError::WrongKeyType(key.to_string())),
            },
            Configuration::Map(map) => match key {
                Key::Array(_) => Err(ConfigurationAccessError::WrongKeyType("".into())), //TODO: Fix this error
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ConfigurationAccessError::KeyNotFound("".into())), //TODO: Fix this error
                },
            },
        }
    }

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationAccessError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationAccessError>,
    {
        match self {
            Configuration::Value(Some(v)) => match T::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            Configuration::Value(None) => Ok(None),
            Configuration::Array(_) => Err(ConfigurationAccessError::UnexpectedNodeType(
                "value", "array",
            )),
            Configuration::Map(_) => {
                Err(ConfigurationAccessError::UnexpectedNodeType("value", "map"))
            }
        }
    }

    pub fn merge(
        previous: Configuration,
        next: Configuration,
    ) -> Result<Configuration, ConfigurationMergeError> {
        match (previous, next) {
            (Configuration::Value(vp), Configuration::Value(vn)) => {
                if TypedValue::is_substitutable_type_option(&vp, &vn) {
                    Ok(Configuration::Value(vn))
                } else {
                    Err(ConfigurationMergeError::IncompatibleValueSubstitution)
                }
            }
            (Configuration::Map(mp), Configuration::Map(mn)) => {
                let merged = Configuration::merge_maps(mp, mn)?;
                Ok(Configuration::Map(merged))
            }
            (Configuration::Array(_), nn @ Configuration::Array(_)) => Ok(nn),
            (_vp, _vm) => Err(ConfigurationMergeError::IncompatibleNodeSubstitution),
        }
    }

    fn merge_maps(
        mut previous: HashMap<String, Configuration>,
        mut next: HashMap<String, Configuration>,
    ) -> Result<HashMap<String, Configuration>, ConfigurationMergeError> {
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
                            return Err(ConfigurationMergeError::IncompatibleValueSubstitution);
                        }
                    }
                    (Configuration::Map(mp), Configuration::Map(mn)) => {
                        let merged = Configuration::merge_maps(mp, mn)?;
                        previous.insert(key, Configuration::Map(merged));
                    }
                    (Configuration::Array(_), nn @ Configuration::Array(_)) => {
                        previous.insert(key, nn);
                    }
                    (_vp, _vn) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                }
            }
        }

        Ok(previous)
    }
}
