use crate::{
    configuration::{CompoundKey, Key, TypedValue},
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
pub enum Node {
    Value(Option<TypedValue>),
    Map(HashMap<String, Node>),
    Array(Vec<Node>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeType {
    Value,
    Map,
    Array,
}

impl Node {
    pub fn get_option<'node, T>(&'node self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'node TypedValue, Error = ConfigurationError>,
    {
        self.get_result(keys).ok().unwrap_or_default()
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

    pub fn get_value<'a, T>(&'a self) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        match self {
            Node::Value(Some(v)) => match TryFrom::try_from(v) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e),
            },
            Node::Value(None) => Ok(None),
            Node::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into())
            }
            Node::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into())
            }
        }
    }

    pub fn descend(&self, key: &Key) -> Result<&Node, ConfigurationError> {
        match self {
            Node::Value(_) => match key {
                Key::Array(_) => {
                    Err(ErrorCode::UnexpectedNodeType(NodeType::Array, NodeType::Value).into())
                }
                Key::Map(_) => {
                    Err(ErrorCode::UnexpectedNodeType(NodeType::Map, NodeType::Value).into())
                }
            },
            Node::Array(array) => match key {
                Key::Array(index) => match array.get(*index) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::IndexOutOfRange(*index).into()),
                },
                Key::Map(inner_key) => Err(ErrorCode::WrongKeyType(inner_key.to_owned()).into()),
            },
            Node::Map(map) => match key {
                Key::Array(i) => Err(ErrorCode::WrongKeyType(i.to_string()).into()),
                Key::Map(k) => match map.get(k) {
                    Some(node) => Ok(node),
                    None => Err(ErrorCode::KeyNotFound(k.to_owned()).into()),
                },
            },
        }
    }

    pub fn try_into<'de, T: DeserializeOwned>(&self) -> Result<T, ConfigurationError> {
        T::deserialize(self)
    }

    pub(crate) fn node_type(&self) -> NodeType {
        match self {
            Node::Value(_) => NodeType::Value,
            Node::Map(_) => NodeType::Map,
            Node::Array(_) => NodeType::Array,
        }
    }
}

pub(crate) fn merge(previous: Node, next: Node) -> Result<Node, ConfigurationError> {
    match (previous, next) {
        (_, vn @ Node::Value(_)) => Ok(vn.clone()),
        (Node::Map(mp), Node::Map(mn)) => Ok(Node::Map(merge_maps(mp, mn)?)),
        (Node::Array(vp), Node::Array(vn)) => Ok(Node::Array(merge_arrays(vp, vn))),
        (vp, vm) => {
            Err(ErrorCode::IncompatibleNodeSubstitution(vp.node_type(), vm.node_type()).into())
        }
    }
}

fn merge_maps<'p>(
    mut previous: HashMap<String, Node>,
    mut next: HashMap<String, Node>,
) -> Result<HashMap<String, Node>, ConfigurationError> {
    for (key, next_node) in next.drain() {
        if !previous.contains_key(&key) {
            previous.insert(key.clone(), next_node.clone());
        } else {
            let previous_node = previous.remove(&key).unwrap();
            match (previous_node, next_node) {
                (Node::Value(_), vn @ Node::Value(_)) => {
                    previous.insert(key.clone(), vn.clone());
                }
                (Node::Map(mp), Node::Map(mn)) => {
                    previous.insert(key.clone(), Node::Map(merge_maps(mp, mn)?));
                }
                (Node::Array(vp), Node::Array(vn)) => {
                    previous.insert(key.clone(), Node::Array(merge_arrays(vp, vn)));
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

fn merge_arrays<'p>(mut vp: Vec<Node>, vn: Vec<Node>) -> Vec<Node> {
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
        $(impl TryFrom<&Node> for $t {
            type Error = ConfigurationError;

            fn try_from(value: &Node) -> Result<Self, Self::Error> {
                match value {
                    Node::Value(Some(tv)) => tv.try_into(),
                    Node::Value(None) => Err(ErrorCode::MissingValue.into()),
                    Node::Map(_) => Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into()),
                    Node::Array(_) => Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into()),
                }
            }
        })*
    };
}

try_from_for!(i8, i16, i32, i64, isize, f32, f64, bool, String);

impl<'conf> TryFrom<&'conf Node> for &'conf str {
    type Error = ConfigurationError;

    fn try_from(value: &'conf Node) -> Result<Self, Self::Error> {
        match value {
            Node::Value(Some(tv)) => tv.try_into(),
            Node::Value(None) => Ok(""),
            Node::Map(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Map).into())
            }
            Node::Array(_) => {
                Err(ErrorCode::UnexpectedNodeType(NodeType::Value, NodeType::Array).into())
            }
        }
    }
}
