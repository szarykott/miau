use crate::{
    configuration::{CompoundKey, Key, TypedValue},
    error::{ConfigurationError, ErrorCode},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    default::Default,
    fmt::Display,
};

#[derive(Debug)]
pub struct Configuration {
    roots: Vec<ConfigurationRoot>,
}

#[derive(Debug)]
pub struct MergedConfiguration {
    root: ConfigurationRoot,
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

impl Configuration {
    pub fn get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        self.roots.iter().rev().find_map(|cr| cr.get::<T>(keys))
    }

    pub(crate) fn add_root(&mut self, root: ConfigurationRoot) {
        self.roots.push(root);
    }

    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        let result = self.merge()?;
        result.try_into()
    }

    pub fn merge(mut self) -> Result<MergedConfiguration, ConfigurationError> {
        let mut roots = self.roots.drain(..);
        let first = roots.next();
        match first {
            Some(cr) => roots
                .try_fold(cr, |acc, next| ConfigurationRoot::merge(acc, next))
                .map(|cr| MergedConfiguration { root: cr }),
            None => Err(ErrorCode::MissingValue.into()),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { roots: Vec::new() }
    }
}

impl MergedConfiguration {
    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        ConfigurationRoot::try_into::<T>(&self.root)
    }
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

impl ConfigurationRoot {
    pub(crate) fn node_type(&self) -> NodeType {
        match self {
            ConfigurationRoot::Value(_) => NodeType::Value,
            ConfigurationRoot::Map(_) => NodeType::Map,
            ConfigurationRoot::Array(_) => NodeType::Array,
        }
    }

    pub fn get_result<'a, T>(&'a self, keys: &CompoundKey) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        let mut node = Result::Ok(self);
        for key in keys {
            node = node.and_then(|nd| nd.descend(key).map_err(|e| e.enrich_with_key(key.clone())));
        }
        node.and_then(|node| node.get_value::<T>())
    }

    pub fn get<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        self.get_result(keys).ok().unwrap_or_default()
    }

    pub fn descend(&self, key: &Key) -> Result<&ConfigurationRoot, ConfigurationError> {
        match self {
            ConfigurationRoot::Value(_) => match key {
                Key::Array(_) => {
                    Err(ErrorCode::UnexpectedNodeType(NodeType::Array, NodeType::Value).into())
                }
                Key::Map(_) => {
                    Err(ErrorCode::UnexpectedNodeType(NodeType::Map, NodeType::Value).into())
                }
            },
            ConfigurationRoot::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(*index).into()),
                },
                Key::Map(inner_key) => Err(ErrorCode::WrongKeyType(inner_key.to_owned()).into()),
            },
            ConfigurationRoot::Map(map) => match key {
                Key::Array(i) => Err(ErrorCode::WrongKeyType(i.to_string()).into()),
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::KeyNotFound(k.to_owned()).into()),
                },
            },
        }
    }

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        match self {
            ConfigurationRoot::Value(Some(v)) => match TryFrom::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            ConfigurationRoot::Value(None) => Ok(None),
            ConfigurationRoot::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into())
            }
            ConfigurationRoot::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into())
            }
        }
    }

    pub fn try_into<'de, T: DeserializeOwned>(&self) -> Result<T, ConfigurationError> {
        T::deserialize(self)
    }

    // MERGING //

    pub fn merge(
        previous: ConfigurationRoot,
        next: ConfigurationRoot,
    ) -> Result<ConfigurationRoot, ConfigurationError> {
        match (previous, next) {
            (_, vn @ ConfigurationRoot::Value(_)) => Ok(vn.clone()),
            (ConfigurationRoot::Map(mp), ConfigurationRoot::Map(mn)) => Ok(ConfigurationRoot::Map(
                ConfigurationRoot::merge_maps(mp, mn)?,
            )),
            (ConfigurationRoot::Array(vp), ConfigurationRoot::Array(vn)) => Ok(
                ConfigurationRoot::Array(ConfigurationRoot::merge_arrays(vp, vn)),
            ),
            (vp, vm) => {
                Err(ErrorCode::IncompatibleNodeSubstitution(vp.node_type(), vm.node_type()).into())
            }
        }
    }

    fn merge_maps<'p>(
        mut previous: HashMap<String, ConfigurationRoot>,
        mut next: HashMap<String, ConfigurationRoot>,
    ) -> Result<HashMap<String, ConfigurationRoot>, ConfigurationError> {
        for (key, next_node) in next.drain() {
            if !previous.contains_key(&key) {
                previous.insert(key.clone(), next_node.clone());
            } else {
                let previous_node = previous.remove(&key).unwrap();
                match (previous_node, next_node) {
                    (ConfigurationRoot::Value(_), vn @ ConfigurationRoot::Value(_)) => {
                        previous.insert(key.clone(), vn.clone());
                    }
                    (ConfigurationRoot::Map(mp), ConfigurationRoot::Map(mn)) => {
                        previous.insert(
                            key.clone(),
                            ConfigurationRoot::Map(ConfigurationRoot::merge_maps(mp, mn)?),
                        );
                    }
                    (ConfigurationRoot::Array(vp), ConfigurationRoot::Array(vn)) => {
                        previous.insert(
                            key.clone(),
                            ConfigurationRoot::Array(ConfigurationRoot::merge_arrays(vp, vn)),
                        );
                    }
                    (vp, vn) => {
                        return Err(ErrorCode::IncompatibleNodeSubstitution(
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

    fn merge_arrays<'p>(
        mut vp: Vec<ConfigurationRoot>,
        vn: Vec<ConfigurationRoot>,
    ) -> Vec<ConfigurationRoot> {
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
}

macro_rules! try_from_for {
    ($($t:ty),*) => {
        $(impl TryFrom<&ConfigurationRoot> for $t {
            type Error = ConfigurationError;

            fn try_from(value: &ConfigurationRoot) -> Result<Self, Self::Error> {
                match value {
                    ConfigurationRoot::Value(Some(tv)) => tv.try_into(),
                    ConfigurationRoot::Value(None) => Err(ErrorCode::MissingValue.into()),
                    ConfigurationRoot::Map(_) => Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into()),
                    ConfigurationRoot::Array(_) => Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into()),
                }
            }
        })*
    };
}

try_from_for!(i8, i16, i32, i64, isize, f32, f64, bool, String);

impl<'conf> TryFrom<&'conf ConfigurationRoot> for &'conf str {
    type Error = ConfigurationError;

    fn try_from(value: &'conf ConfigurationRoot) -> Result<Self, Self::Error> {
        match value {
            ConfigurationRoot::Value(Some(tv)) => tv.try_into(),
            ConfigurationRoot::Value(None) => Ok(""),
            ConfigurationRoot::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into())
            }
            ConfigurationRoot::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into())
            }
        }
    }
}
