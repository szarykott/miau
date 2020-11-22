use crate::{
    configuration::{
        common, node::ConfigurationNode, CompoundKey, ConfigurationDefinition, ConfigurationInfo,
        Lens, Value,
    },
    error::ConfigurationError,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    default::Default,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "ConfigurationNode")]
pub struct Configuration {
    pub(crate) roots: Vec<ConfigurationDefinition>,
}

impl Configuration {
    pub fn new_singular(info: ConfigurationInfo, root: ConfigurationNode) -> Self {
        Configuration {
            roots: vec![ConfigurationDefinition::new(info, root)],
        }
    }

    pub fn new_empty() -> Self {
        Configuration { roots: vec![] }
    }

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
        common::get_result_internal(self.roots.iter().map(|def| &def.root), &keys)
    }

    pub fn lens(&'_ self) -> Lens<'_> {
        self.into()
    }

    pub fn try_convert_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_owned().and_then(|node| node.try_convert_into())
    }

    pub fn merge_owned(mut self) -> Result<ConfigurationNode, ConfigurationError> {
        common::merge_owned(self.roots.drain(..).map(|def| def.root))
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { roots: Vec::new() }
    }
}

impl From<ConfigurationNode> for Configuration {
    fn from(node: ConfigurationNode) -> Self {
        Configuration::new_singular(ConfigurationInfo::new("other Node", "unknown"), node)
    }
}

impl From<HashMap<String, String>> for Configuration {
    fn from(map: HashMap<String, String>) -> Self {
        let mut result = HashMap::new();

        for (k, v) in map {
            result.insert(k, ConfigurationNode::Value(Some(Value::String(v))));
        }

        let node = ConfigurationNode::Map(result);
        Configuration::new_singular(ConfigurationInfo::new("HashMap", "HashMap"), node)
    }
}
