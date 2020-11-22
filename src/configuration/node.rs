use crate::{
    configuration::{CompoundKey, Key, Value},
    error::{ConfigurationError, ErrorCode},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Display,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigurationNode {
    Value(Option<Value>),
    Map(HashMap<String, ConfigurationNode>),
    Array(Vec<ConfigurationNode>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeType {
    Value,
    Map,
    Array,
}

impl ConfigurationNode {
    pub fn get<'node, T, K>(&'node self, keys: K) -> Option<T>
    where
        T: TryFrom<&'node Value, Error = ConfigurationError>,
        K: TryInto<CompoundKey>,
    {
        let keys = keys.try_into().ok()?;
        self.get_result_internal(&keys).ok().unwrap_or_default()
    }

    pub fn get_result<'a, T, K>(&'a self, keys: K) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        K: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        let keys = keys.try_into()?;
        self.get_result_internal(&keys)
    }

    pub(crate) fn get_result_internal<'a, T>(
        &'a self,
        keys: &CompoundKey,
    ) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
    {
        self.descend_many(keys)
            .and_then(|node| node.get_value::<T>().map_err(|e| e.enrich_with_keys(keys)))
    }

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
    {
        match self {
            ConfigurationNode::Value(Some(v)) => match TryFrom::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            ConfigurationNode::Value(None) => Ok(None),
            ConfigurationNode::Array(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into())
            }
            ConfigurationNode::Map(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into())
            }
        }
    }

    pub fn descend_many(
        &self,
        keys: &CompoundKey,
    ) -> Result<&ConfigurationNode, ConfigurationError> {
        Result::Ok(self).and_then(|n| n.descend_iter(keys.iter()))
    }

    fn descend_iter<'a>(
        &self,
        mut kiter: impl Iterator<Item = &'a Key>,
    ) -> Result<&ConfigurationNode, ConfigurationError> {
        match kiter.next() {
            Some(key) => self
                .descend(key)
                .and_then(|n| n.descend_iter(kiter))
                .map_err(|e| e.enrich_with_key(key.clone())),
            None => Ok(self),
        }
    }

    pub fn descend(&self, key: &Key) -> Result<&ConfigurationNode, ConfigurationError> {
        match self {
            ConfigurationNode::Value(_) => match key {
                Key::Array(_) => {
                    Err(ErrorCode::WrongNodeType(NodeType::Array, NodeType::Value).into())
                }
                Key::Map(_) => Err(ErrorCode::WrongNodeType(NodeType::Map, NodeType::Value).into()),
            },
            ConfigurationNode::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(*index).into()),
                },
                Key::Map(inner_key) => {
                    Err(ErrorCode::WrongKeyType(NodeType::Array, inner_key.to_owned()).into())
                }
            },
            ConfigurationNode::Map(map) => match key {
                Key::Array(i) => Err(ErrorCode::WrongKeyType(NodeType::Map, i.to_string()).into()),
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::KeyNotFound(k.to_owned()).into()),
                },
            },
        }
    }

    pub fn try_convert_into<T: DeserializeOwned>(&'_ self) -> Result<T, ConfigurationError> {
        T::deserialize(self).map_err(|e| {
            e.enrich_with_context(format!(
                "Failed to deserialize configuration to type {}",
                std::any::type_name::<T>()
            ))
        })
    }

    pub fn node_type(&self) -> NodeType {
        match self {
            ConfigurationNode::Value(_) => NodeType::Value,
            ConfigurationNode::Map(_) => NodeType::Map,
            ConfigurationNode::Array(_) => NodeType::Array,
        }
    }
}

pub fn merge(
    previous: ConfigurationNode,
    next: ConfigurationNode,
) -> Result<ConfigurationNode, ConfigurationError> {
    match (previous, next) {
        (_, vn @ ConfigurationNode::Value(_)) => Ok(vn),
        (ConfigurationNode::Map(mp), ConfigurationNode::Map(mn)) => {
            Ok(ConfigurationNode::Map(merge_maps(mp, mn)?))
        }
        (ConfigurationNode::Array(vp), ConfigurationNode::Array(vn)) => {
            Ok(ConfigurationNode::Array(merge_arrays(vp, vn)))
        }
        (vp, vm) => Err(ErrorCode::BadNodeMerge(vp.node_type(), vm.node_type()).into()),
    }
}

fn merge_maps(
    mut previous: HashMap<String, ConfigurationNode>,
    mut next: HashMap<String, ConfigurationNode>,
) -> Result<HashMap<String, ConfigurationNode>, ConfigurationError> {
    for (key, next_node) in next.drain() {
        if !previous.contains_key(&key) {
            previous.insert(key.clone(), next_node.clone());
        } else {
            let previous_node = previous.remove(&key).unwrap();
            match (previous_node, next_node) {
                (ConfigurationNode::Value(_), vn @ ConfigurationNode::Value(_)) => {
                    previous.insert(key.clone(), vn.clone());
                }
                (ConfigurationNode::Map(mp), ConfigurationNode::Map(mn)) => {
                    previous.insert(
                        key.clone(),
                        ConfigurationNode::Map(
                            merge_maps(mp, mn)
                                .map_err(|e| e.enrich_with_key(Key::Map(key.clone())))?,
                        ),
                    );
                }
                (ConfigurationNode::Array(vp), ConfigurationNode::Array(vn)) => {
                    previous.insert(key.clone(), ConfigurationNode::Array(merge_arrays(vp, vn)));
                }
                (vp, vn) => {
                    let error: ConfigurationError =
                        ErrorCode::BadNodeMerge(vp.node_type(), vn.node_type()).into();

                    return Err(error
                        .enrich_with_context("Failed to merge maps")
                        .enrich_with_key(Key::Map(key)));
                }
            }
        }
    }

    Ok(previous)
}

fn merge_arrays(
    mut vp: Vec<ConfigurationNode>,
    vn: Vec<ConfigurationNode>,
) -> Vec<ConfigurationNode> {
    if vp.len() >= vn.len() {
        for (index, root) in vn.iter().enumerate() {
            vp[index] = root.clone();
        }
    } else {
        vp.clear();
        for e in vn.iter() {
            vp.push(e.clone())
        }
    }

    vp
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Value => write!(f, "value"),
            NodeType::Map => write!(f, "map"),
            NodeType::Array => write!(f, "array"),
        }
    }
}

macro_rules! try_from_for {
    ($($t:ty),*) => {
        $(impl TryFrom<&ConfigurationNode> for $t {
            type Error = ConfigurationError;

            fn try_from(value: &ConfigurationNode) -> Result<Self, Self::Error> {
                match value {
                    ConfigurationNode::Value(Some(tv)) => tv.try_into(),
                    ConfigurationNode::Value(None) => Err(ErrorCode::NullValue.into()),
                    ConfigurationNode::Map(_) => Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into()),
                    ConfigurationNode::Array(_) => Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into()),
                }
            }
        })*
    };
}

try_from_for!(i8, i16, i32, i64, isize, f32, f64, bool, String);

impl<'conf> TryFrom<&'conf ConfigurationNode> for &'conf str {
    type Error = ConfigurationError;

    fn try_from(value: &'conf ConfigurationNode) -> Result<Self, Self::Error> {
        match value {
            ConfigurationNode::Value(Some(tv)) => tv.try_into(),
            ConfigurationNode::Value(None) => Ok(""),
            ConfigurationNode::Map(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into())
            }
            ConfigurationNode::Array(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into())
            }
        }
    }
}
