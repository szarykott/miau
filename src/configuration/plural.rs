use crate::{
    configuration::{
        common, CompoundKey, ConfigurationDefinition, ConfigurationInfo, ConfigurationTree, Lens,
        Value,
    },
    error::ConfigurationError,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    default::Default,
};

/// Owns configuration trees built from all sources.
///
/// Allows retrieving data stored in all trees, merging them, creating lenses and converting into strongly typed structs.
/// `Configuration` should only be created by builder or sources. Most functions are public to server most bizzare needs.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "ConfigurationTree")]
pub struct Configuration {
    pub(crate) roots: Vec<ConfigurationDefinition>,
}

impl Configuration {
    /// Creates new `Configuration` from one tree and its associated information.
    pub fn new_singular(info: ConfigurationInfo, root: ConfigurationTree) -> Self {
        Configuration {
            roots: vec![ConfigurationDefinition::new(info, root)],
        }
    }

    /// Creates new empty `Configuration`.
    pub fn new_empty() -> Self {
        Configuration { roots: vec![] }
    }

    /// Retrieves value stored in `Configuration` under given `keys`.
    ///
    /// If no value is found or key transformation fails `None` is returned.
    /// [`get_result`](Self::get_result) provides more insight into root cause of error.
    ///
    /// # Example
    /// TODO: add example
    pub fn get<'a, T, S>(&'a self, keys: S) -> Option<T>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        self.get_result_internal(keys.try_into().ok()?)
            .unwrap_or_default()
    }

    /// Retrieves value stored in `Configuration` under given `keys`.
    ///
    /// If key transformation fails error is returned. Value is returned if found, `None` otherwise.
    ///
    /// # Example
    /// TODO: add example
    pub fn get_result<'a, T, S>(&'a self, keys: S) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        self.get_result_internal(keys.try_into()?)
    }

    fn get_result_internal<'a, T>(
        &'a self,
        keys: CompoundKey,
    ) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'a Value, Error = ConfigurationError>,
    {
        common::get_result_internal(self.roots.iter().map(|def| &def.root), &keys)
    }

    /// Creates `Lens` from this `Configuration`.
    pub fn lens(&'_ self) -> Lens<'_> {
        self.into()
    }

    /// Deserializes `Configuration` into strongly typed struct.
    ///
    /// It is only required that struct to be deserialized to implements `Deserialize`
    /// and contains no borrowed fields, for instance `&str`.
    /// Due to memory model of `miau` it is impossible to deserialize into such fields.
    pub fn try_convert_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_owned().and_then(|node| node.try_convert_into())
    }

    /// Merges trees contained in `Configuration` into one tree by consuming them.
    ///
    /// To merge cloned, invoke [`lens`](Self::lens) function first.
    pub fn merge_owned(mut self) -> Result<ConfigurationTree, ConfigurationError> {
        common::merge_owned(self.roots.drain(..).map(|def| def.root))
    }

    /// Retrives information about configuration trees, in order used internally by `Configuration`.
    ///
    ///```rust
    ///# use miau::configuration::Configuration;
    ///let configuration = Configuration::default(); // normally populated configuration should be used!
    ///
    ///for info in configuration.infos() {
    ///     println!("{}", info);
    ///}
    ///```
    pub fn infos(&self) -> impl Iterator<Item = &ConfigurationInfo> {
        self.roots.iter().map(|def| &def.info)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { roots: Vec::new() }
    }
}

impl From<ConfigurationTree> for Configuration {
    fn from(node: ConfigurationTree) -> Self {
        Configuration::new_singular(ConfigurationInfo::new("other Tree", "unknown"), node)
    }
}

impl From<HashMap<String, String>> for Configuration {
    fn from(map: HashMap<String, String>) -> Self {
        let mut result = HashMap::new();

        for (k, v) in map {
            result.insert(k, ConfigurationTree::Value(Some(Value::String(v))));
        }

        let node = ConfigurationTree::Map(result);
        Configuration::new_singular(ConfigurationInfo::new("HashMap", "HashMap"), node)
    }
}
