use crate::{
    configuration::{node, node::Node, CompoundKey, Value},
    error::{ConfigurationError, ErrorCode},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    default::Default,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "Node")]
pub struct Configuration {
    pub(crate) roots: Vec<Node>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SingularConfiguration {
    pub(crate) root: Node,
}

impl Configuration {
    pub fn get_option<'a, T, S>(&'a self, keys: S) -> Option<T>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        let keys = keys.try_into().ok()?;
        self.roots
            .iter()
            .rev()
            .find_map(|node| node.get_option::<T>(&keys))
    }

    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_owned()?.try_into()
    }

    pub fn merge_owned(mut self) -> Result<SingularConfiguration, ConfigurationError> {
        let mut roots = self.roots.drain(..);
        match roots.next() {
            Some(node) => roots
                .try_fold(node, |acc, next| node::merge(acc, next))
                .map(|final_node| SingularConfiguration { root: final_node }),
            None => Err(ErrorCode::MissingValue.into()),
        }
    }
}

impl SingularConfiguration {
    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        Node::try_into::<T>(&self.root)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { roots: Vec::new() }
    }
}

impl From<Node> for Configuration {
    fn from(node: Node) -> Self {
        Configuration { roots: vec![node] }
    }
}

impl From<Node> for SingularConfiguration {
    fn from(node: Node) -> Self {
        SingularConfiguration { root: node }
    }
}
