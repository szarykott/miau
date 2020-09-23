use crate::error::{ConfigurationAccessError, ConfigurationMergeError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Configuration {
    pub root: ConfigurationNode,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigurationNode {
    Value(Option<TypedValue>),
    Map(HashMap<String, ConfigurationNode>),
    Array(Vec<ConfigurationNode>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypedValue {
    String(String),
    Bool(bool),
    SignedInteger(u64),
    UnsignedInteger(i64),
    Float(f64),
}

impl TypedValue {
    fn underlying_type(&self) -> &'static str {
        match self {
            TypedValue::String(_) => "String",
            TypedValue::Bool(_) => "bool",
            TypedValue::SignedInteger(_) => "signed integer",
            TypedValue::UnsignedInteger(_) => "unsigned integer",
            TypedValue::Float(_) => "float",
        }
    }
}

macro_rules! impl_try_from {
    ($e:path => $($t:ty),*) => {
        $(impl TryFrom<&TypedValue> for $t {
            type Error = ConfigurationAccessError;

            fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                if let $e(v) = value {
                    Ok(*v as $t)
                } else {
                    Err(ConfigurationAccessError::UnexpectedValueType(stringify!($t), value.underlying_type()))
                }
            }
        })*
    };
}

impl_try_from!(TypedValue::SignedInteger => i8, i16, i32, i64, isize);
impl_try_from!(TypedValue::UnsignedInteger => u8, u16, u32, u64, usize);
impl_try_from!(TypedValue::Float => f32, f64);
impl_try_from!(TypedValue::Bool => bool);

impl<'a> TryFrom<&'a TypedValue> for &'a str {
    type Error = ConfigurationAccessError;

    fn try_from(value: &'a TypedValue) -> Result<Self, Self::Error> {
        if let TypedValue::String(v) = value {
            Ok(v.as_str())
        } else {
            Err(ConfigurationAccessError::UnexpectedValueType(
                "String",
                value.underlying_type(),
            ))
        }
    }
}

impl TryFrom<&TypedValue> for String {
    type Error = ConfigurationAccessError;

    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
        if let TypedValue::String(v) = value {
            Ok(v.to_string())
        } else {
            Err(ConfigurationAccessError::UnexpectedValueType(
                "String",
                value.underlying_type(),
            ))
        }
    }
}

impl Configuration {
    pub fn merge(
        previous: Configuration,
        next: Configuration,
    ) -> Result<Configuration, ConfigurationMergeError> {
        match (previous.root, next.root) {
            (ConfigurationNode::Value(_), nn @ ConfigurationNode::Value(_)) => {
                // TODO: Handle type-check during substitution
                Ok(Configuration { root: nn })
            }
            (ConfigurationNode::Value(_), ConfigurationNode::Map(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Value(_), ConfigurationNode::Array(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Map(_), ConfigurationNode::Value(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Map(mp), ConfigurationNode::Map(mn)) => {
                let merged = Configuration::merge_maps(mp, mn)?;
                Ok(Configuration {
                    root: ConfigurationNode::Map(merged),
                })
            }
            (ConfigurationNode::Map(_), ConfigurationNode::Array(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Array(_), ConfigurationNode::Value(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Array(_), ConfigurationNode::Map(_)) => {
                Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
            }
            (ConfigurationNode::Array(_), nn @ ConfigurationNode::Array(_)) => {
                Ok(Configuration { root: nn })
            }
        }
    }

    fn merge_maps(
        mut previous: HashMap<String, ConfigurationNode>,
        mut next: HashMap<String, ConfigurationNode>,
    ) -> Result<HashMap<String, ConfigurationNode>, ConfigurationMergeError> {
        for (key, next_node) in next.drain() {
            if !previous.contains_key(&key) {
                previous.insert(key, next_node);
            } else {
                let previous_node = previous.remove(&key).unwrap();
                match (previous_node, next_node) {
                    (ConfigurationNode::Value(_), nn @ ConfigurationNode::Value(_)) => {
                        // TODO: Handle type-check during substitution
                        previous.insert(key, nn);
                    }
                    (ConfigurationNode::Value(_), ConfigurationNode::Map(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Value(_), ConfigurationNode::Array(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Map(_), ConfigurationNode::Value(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Map(mp), ConfigurationNode::Map(mn)) => {
                        let merged = Configuration::merge_maps(mp, mn)?;
                        previous.insert(key, ConfigurationNode::Map(merged));
                    }
                    (ConfigurationNode::Map(_), ConfigurationNode::Array(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Array(_), ConfigurationNode::Value(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Array(_), ConfigurationNode::Map(_)) => {
                        return Err(ConfigurationMergeError::IncompatibleNodeSubstitution)
                    }
                    (ConfigurationNode::Array(_), nn @ ConfigurationNode::Array(_)) => {
                        previous.insert(key, nn);
                    }
                }
            }
        }

        Ok(previous)
    }
}

impl ConfigurationNode {
    pub fn get(&self, key: &str) -> Result<&ConfigurationNode, ConfigurationAccessError> {
        match self {
            ConfigurationNode::Value(_) => Err(ConfigurationAccessError::UnexpectedNodeType(
                "not value",
                "value",
            )),
            ConfigurationNode::Array(array) => match key.parse::<usize>() {
                Ok(index) => match array.get(index) {
                    Some(node) => Ok(node),
                    None => Err(ConfigurationAccessError::IndexOutOfRange(index)),
                },
                Err(e) => Err(ConfigurationAccessError::WrongKeyType(e.to_string())),
            },
            ConfigurationNode::Map(map) => match map.get(key) {
                Some(node) => Ok(node),
                None => Err(ConfigurationAccessError::KeyNotFound(key.into())),
            },
        }
    }

    pub fn get_array(&self, index: usize) -> Result<&ConfigurationNode, ConfigurationAccessError> {
        match self {
            ConfigurationNode::Value(_) => Err(ConfigurationAccessError::UnexpectedNodeType(
                "array", "value",
            )),
            ConfigurationNode::Map(_) => {
                Err(ConfigurationAccessError::UnexpectedNodeType("array", "map"))
            }
            ConfigurationNode::Array(array) => match array.get(index) {
                Some(node) => Ok(node),
                None => Err(ConfigurationAccessError::IndexOutOfRange(index)),
            },
        }
    }

    pub fn get_map(&self, key: &str) -> Result<&ConfigurationNode, ConfigurationAccessError> {
        match self {
            ConfigurationNode::Value(_) => {
                Err(ConfigurationAccessError::UnexpectedNodeType("map", "value"))
            }
            ConfigurationNode::Array(_) => {
                Err(ConfigurationAccessError::UnexpectedNodeType("map", "array"))
            }
            ConfigurationNode::Map(map) => match map.get(key) {
                Some(node) => Ok(node),
                None => Err(ConfigurationAccessError::KeyNotFound(key.into())),
            },
        }
    }

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationAccessError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationAccessError>,
    {
        match self {
            ConfigurationNode::Value(Some(v)) => match T::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            ConfigurationNode::Value(None) => Ok(None),
            ConfigurationNode::Array(_) => Err(ConfigurationAccessError::UnexpectedNodeType(
                "value", "array",
            )),
            ConfigurationNode::Map(_) => {
                Err(ConfigurationAccessError::UnexpectedNodeType("value", "map"))
            }
        }
    }
}
