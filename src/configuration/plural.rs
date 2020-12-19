use crate::{
    configuration::{
        common, CompoundKey, ConfigurationDefinition, ConfigurationInfo, ConfigurationRead,
        ConfigurationTree, Lens, Value,
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
///
/// To read values from `Configuration` you need to pull [`ConfigurationRead`](super::ConfigurationRead) in scope.
/// # Example
///```rust
///use miau::configuration::{Configuration, ConfigurationRead};
///
///let configuration = Configuration::default(); //  aka empty
///let word: Option<String> = configuration.get("word");
///assert_eq!(None, word);
///```
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

impl<'config, T, K> ConfigurationRead<'config, T, K> for Configuration
where
    T: TryFrom<&'config Value, Error = ConfigurationError>,
    K: TryInto<CompoundKey, Error = ConfigurationError>,
{
    fn get_result(&'config self, keys: K) -> Result<Option<T>, ConfigurationError> {
        let keys = keys.try_into()?;
        common::get_result_internal(self.roots.iter().map(|def| &def.root), &keys)
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
