use crate::{
    configuration::{node, node::Node, CompoundKey, TypedValue},
    error::{ConfigurationError, ErrorCode},
};
use serde::de::DeserializeOwned;
use std::{convert::TryFrom, default::Default};

#[derive(Debug)]
pub struct Configuration {
    roots: Vec<Node>,
}

#[derive(Debug)]
pub struct MergedConfiguration {
    root: Node,
}

impl Configuration {
    pub(crate) fn add_root(&mut self, root: Node) {
        self.roots.push(root);
    }

    pub fn get_option<'a, T>(&'a self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'a TypedValue, Error = ConfigurationError>,
    {
        self.roots
            .iter()
            .rev()
            .find_map(|node| node.get_option::<T>(keys))
    }

    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_owned()?.try_into()
    }

    pub fn merge_owned(mut self) -> Result<MergedConfiguration, ConfigurationError> {
        let mut roots = self.roots.drain(..);
        match roots.next() {
            Some(node) => roots
                .try_fold(node, |acc, next| node::merge(acc, next))
                .map(|final_node| MergedConfiguration { root: final_node }),
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
        Node::try_into::<T>(&self.root)
    }
}
