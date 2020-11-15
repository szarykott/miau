use crate::{
    configuration::{node, node::Node, CompoundKey, SingularConfiguration, Value},
    error::{ConfigurationError, ErrorCode},
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    default::Default,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "Node")]
pub struct Configuration {
    pub(crate) roots: Vec<Node>,
}

impl Configuration {
    pub fn get<'a, T, S>(&'a self, keys: S) -> Option<T>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        self.get_result_internal(keys.try_into().ok()?)
            .unwrap_or_default()
    }

    pub fn get_result<'a, T, S>(&'a self, keys: S) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        self.get_result_internal(keys.try_into()?)
    }

    /// Internal method is used so that `get_option` can not specify error of TryInto
    fn get_result_internal<'a, T>(
        &'a self,
        keys: CompoundKey,
    ) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
    {
        for candidate in self.roots.iter().rev() {
            if let result @ Ok(_) = candidate.get_result::<T>(&keys) {
                return result;
            }
        }

        Ok(None)
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
            None => {
                let error: ConfigurationError = ErrorCode::EmptyConfiguration.into();
                Err(error.enrich_with_context("Failed to merge configurations"))
            }
        }
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

impl From<HashMap<String, String>> for Configuration {
    fn from(map: HashMap<String, String>) -> Self {
        let mut result = HashMap::new();

        for (k, v) in map {
            result.insert(k, Node::Value(Some(Value::String(v))));
        }

        Configuration {
            roots: vec![Node::Map(result)],
        }
    }
}
