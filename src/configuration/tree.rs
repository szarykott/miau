use crate::{
    configuration::{CompoundKey, ConfigurationRead, Key, Value},
    error::{ConfigurationError, ErrorCode},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Display,
};

/// Stores information from single configuration source.
///
/// It is a tree whoose nodes can be maps or arrays of other trees and leaves are values.
///
/// To read values from `ConfigurationTree` you need to pull [`ConfigurationRead`](super::ConfigurationRead) in scope.
/// # Example
///```rust
///use miau::configuration::{Configuration, ConfigurationRead, Value, ConfigurationTree};
///
/// // just for demonstration purpose, you should rely on serde and Configuration to get reference to tree
///let configuration = ConfigurationTree::Value(None);
///
///let word: Option<String> = configuration.get("word");
///assert_eq!(None, word);
///```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigurationTree {
    /// Configuration value
    Value(Option<Value>),
    /// Map of other configuration tress.
    Map(HashMap<String, ConfigurationTree>),
    /// Array of other configuration trees.
    Array(Vec<ConfigurationTree>),
}

/// Describes node type of current [`ConfigurationTree`].
#[derive(Debug, Eq, PartialEq)]
pub enum NodeType {
    /// Node type of [`ConfigurationTree::Value`]
    Value,
    /// Node type of [`ConfigurationTree::Map`]
    Map,
    /// Node type of [`ConfigurationTree::Array`]
    Array,
}

impl ConfigurationTree {
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

    /// Retrieves strongly typed primitive value from this ConfigurationTree.
    ///
    /// This operation can only be succesfull if this is a leaf of the tree.
    /// To retrive complex structures, for instance vectors or user defined structs, use [`try_convert_into`](Self::try_convert_into).
    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
    {
        match self {
            ConfigurationTree::Value(Some(v)) => match TryFrom::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            ConfigurationTree::Value(None) => Ok(None),
            ConfigurationTree::Array(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into())
            }
            ConfigurationTree::Map(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into())
            }
        }
    }

    /// Deserializes `ConfigurationTree` into strongly typed struct.
    ///
    /// It is only required that struct to be deserialized to implements [`Deserialize`](serde::Deserialize)
    /// and contains no borrowed fields, for instance `&str`.
    /// Due to memory model of `miau` it is impossible to deserialize into such fields.
    pub fn try_convert_into<T: DeserializeOwned>(&'_ self) -> Result<T, ConfigurationError> {
        T::deserialize(self).map_err(|e| {
            e.enrich_with_context(format!(
                "Failed to deserialize configuration to type {}",
                std::any::type_name::<T>()
            ))
        })
    }

    pub(crate) fn node_type(&self) -> NodeType {
        match self {
            ConfigurationTree::Value(_) => NodeType::Value,
            ConfigurationTree::Map(_) => NodeType::Map,
            ConfigurationTree::Array(_) => NodeType::Array,
        }
    }

    pub(crate) fn descend_many(
        &self,
        keys: &CompoundKey,
    ) -> Result<&ConfigurationTree, ConfigurationError> {
        Result::Ok(self).and_then(|n| n.descend_iter(keys.iter()))
    }

    fn descend_iter<'a>(
        &self,
        mut kiter: impl Iterator<Item = &'a Key>,
    ) -> Result<&ConfigurationTree, ConfigurationError> {
        match kiter.next() {
            Some(key) => self
                .descend(key)
                .and_then(|n| n.descend_iter(kiter))
                .map_err(|e| e.enrich_with_key(key.clone())),
            None => Ok(self),
        }
    }

    pub(crate) fn descend(&self, key: &Key) -> Result<&ConfigurationTree, ConfigurationError> {
        match self {
            ConfigurationTree::Value(_) => match key {
                Key::Array(_) => {
                    Err(ErrorCode::WrongNodeType(NodeType::Array, NodeType::Value).into())
                }
                Key::Map(_) => Err(ErrorCode::WrongNodeType(NodeType::Map, NodeType::Value).into()),
            },
            ConfigurationTree::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(*index).into()),
                },
                Key::Map(inner_key) => {
                    Err(ErrorCode::WrongKeyType(NodeType::Array, inner_key.to_owned()).into())
                }
            },
            ConfigurationTree::Map(map) => match key {
                Key::Array(i) => Err(ErrorCode::WrongKeyType(NodeType::Map, i.to_string()).into()),
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::KeyNotFound(k.to_owned()).into()),
                },
            },
        }
    }
}

impl<'config, T, K> ConfigurationRead<'config, T, K> for ConfigurationTree
where
    T: TryFrom<&'config Value, Error = ConfigurationError>,
    K: TryInto<CompoundKey, Error = ConfigurationError>,
{
    fn get_result(&'config self, keys: K) -> Result<Option<T>, ConfigurationError> {
        let keys = keys.try_into()?;
        self.get_result_internal(&keys)
    }
}

pub(crate) fn merge(
    previous: ConfigurationTree,
    next: ConfigurationTree,
) -> Result<ConfigurationTree, ConfigurationError> {
    match (previous, next) {
        (_, vn @ ConfigurationTree::Value(_)) => Ok(vn),
        (ConfigurationTree::Map(mp), ConfigurationTree::Map(mn)) => {
            Ok(ConfigurationTree::Map(merge_maps(mp, mn)?))
        }
        (ConfigurationTree::Array(vp), ConfigurationTree::Array(vn)) => {
            Ok(ConfigurationTree::Array(merge_arrays(vp, vn)))
        }
        (vp, vm) => Err(ErrorCode::BadNodeMerge(vp.node_type(), vm.node_type()).into()),
    }
}

fn merge_maps(
    mut previous: HashMap<String, ConfigurationTree>,
    mut next: HashMap<String, ConfigurationTree>,
) -> Result<HashMap<String, ConfigurationTree>, ConfigurationError> {
    for (key, next_node) in next.drain() {
        if !previous.contains_key(&key) {
            previous.insert(key.clone(), next_node.clone());
        } else {
            let previous_node = previous.remove(&key).unwrap();
            match (previous_node, next_node) {
                (ConfigurationTree::Value(_), vn @ ConfigurationTree::Value(_)) => {
                    previous.insert(key.clone(), vn.clone());
                }
                (ConfigurationTree::Map(mp), ConfigurationTree::Map(mn)) => {
                    previous.insert(
                        key.clone(),
                        ConfigurationTree::Map(
                            merge_maps(mp, mn)
                                .map_err(|e| e.enrich_with_key(Key::Map(key.clone())))?,
                        ),
                    );
                }
                (ConfigurationTree::Array(vp), ConfigurationTree::Array(vn)) => {
                    previous.insert(key.clone(), ConfigurationTree::Array(merge_arrays(vp, vn)));
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
    mut vp: Vec<ConfigurationTree>,
    vn: Vec<ConfigurationTree>,
) -> Vec<ConfigurationTree> {
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
        $(impl TryFrom<&ConfigurationTree> for $t {
            type Error = ConfigurationError;

            fn try_from(value: &ConfigurationTree) -> Result<Self, Self::Error> {
                match value {
                    ConfigurationTree::Value(Some(tv)) => tv.try_into(),
                    ConfigurationTree::Value(None) => Err(ErrorCode::NullValue.into()),
                    ConfigurationTree::Map(_) => Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into()),
                    ConfigurationTree::Array(_) => Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into()),
                }
            }
        })*
    };
}

try_from_for!(i8, i16, i32, i64, isize, f32, f64, bool, String);

impl<'conf> TryFrom<&'conf ConfigurationTree> for &'conf str {
    type Error = ConfigurationError;

    fn try_from(value: &'conf ConfigurationTree) -> Result<Self, Self::Error> {
        match value {
            ConfigurationTree::Value(Some(tv)) => tv.try_into(),
            ConfigurationTree::Value(None) => Ok(""),
            ConfigurationTree::Map(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Map).into())
            }
            ConfigurationTree::Array(_) => {
                Err(ErrorCode::WrongNodeType(NodeType::Value, NodeType::Array).into())
            }
        }
    }
}
