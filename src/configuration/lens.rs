use super::{
    common, CompoundKey, Configuration, ConfigurationDefinition, ConfigurationDefinitionLens,
    ConfigurationRead, ConfigurationTree, Value,
};
use crate::error::ConfigurationError;
use serde::de::DeserializeOwned;
use std::convert::{From, TryFrom, TryInto};

/// Provides lensing capabilities to a configuration reader.
///
/// Allows scoping into configuration section of choice for read only access.
/// It can be seen as a borrowed version of [`Configuration`](super::Configuration).
///
/// # Example
/// TODO: Add example
#[derive(Debug)]
pub struct Lens<'config> {
    roots: Vec<ConfigurationDefinitionLens<'config>>,
}

impl<'config> Lens<'config> {
    /// Creates new instance of `Lens` from a single configuration definition.
    ///
    /// Such lens might be useful to scope into configurations section of interest to reduce path length needed to retrive values.
    pub fn new_singular(def: &'config ConfigurationDefinition) -> Self {
        Lens {
            roots: vec![def.into()],
        }
    }

    /// Creates new instance of `Lens` from [`Configuration`](super::Configuration).
    ///
    /// It enables lensing into multiple configuration trees at once.
    /// It handles situation like possibility of absence of given subtree in subset of configuration trees.
    /// When substree is missing, given tree is simply ignored in lens.
    pub fn new(config: &'config Configuration) -> Self {
        Lens {
            roots: config.roots.iter().map(|r| r.into()).collect(),
        }
    }

    /// Attempts to lens into given `Lens`
    ///
    /// Function can only return error if transformation of `keys` failed.
    /// If none of configuration trees contains requested key, empty `Lens` will be returned.
    pub fn try_lens<S>(&self, keys: S) -> Result<Self, ConfigurationError>
    where
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        let keys = keys.try_into()?;

        let new_roots = self
            .roots
            .iter()
            .map(|def| def.mutate(|node| node.descend_many(&keys).ok()))
            .collect();

        Ok(Lens { roots: new_roots })
    }

    /// Deserializes `Lens` into strongly typed struct.
    ///
    /// It is only required that struct to be deserialized to implements `Deserialize`
    /// and contains no borrowed fields, for instance `&str`.
    /// Due to memory model of `miau` it is impossible to deserialize into such fields.
    pub fn try_convert_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_cloned().and_then(|node| node.try_convert_into())
    }

    /// Merges trees contained in `Lens` into one tree by cloning them.
    pub fn merge_cloned(mut self) -> Result<ConfigurationTree, ConfigurationError> {
        common::merge_cloned(self.roots.drain(..).filter_map(|def| def.node))
    }
}

impl<'config, T, K> ConfigurationRead<'config, T, K> for Lens<'config>
where
    T: TryFrom<&'config Value, Error = ConfigurationError>,
    K: TryInto<CompoundKey, Error = ConfigurationError>,
{
    fn get_result(&'config self, keys: K) -> Result<Option<T>, ConfigurationError> {
        let keys = keys.try_into()?;
        common::get_result_internal(self.roots.iter().filter_map(|def| def.node), &keys)
    }
}

impl<'config> From<&'config ConfigurationDefinition> for Lens<'config> {
    fn from(config: &'config ConfigurationDefinition) -> Self {
        Lens::new_singular(config)
    }
}

impl<'config> From<&'config Configuration> for Lens<'config> {
    fn from(config: &'config Configuration) -> Self {
        Lens::new(config)
    }
}
