use super::{
    common, CompoundKey, Configuration, ConfigurationDefinition, ConfigurationDefinitionLens,
    ConfigurationNode, Value,
};
use crate::error::ConfigurationError;
use serde::de::DeserializeOwned;
use std::convert::{From, TryFrom, TryInto};

/// Provides lensing capabilities to a configuration reader.
/// Allows scoping into configuration section of choice for read only access.
#[derive(Debug)]
pub struct Lens<'config> {
    roots: Vec<ConfigurationDefinitionLens<'config>>,
}

impl<'config> Lens<'config> {
    pub fn new_singular(def: &'config ConfigurationDefinition) -> Self {
        Lens {
            roots: vec![def.into()],
        }
    }

    pub fn new(config: &'config Configuration) -> Self {
        Lens {
            roots: config.roots.iter().map(|r| r.into()).collect(),
        }
    }

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

    pub fn get<T, S>(&'config self, keys: S) -> Option<T>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        self.get_result_internal(&keys.try_into().ok()?)
            .unwrap_or_default()
    }

    pub fn get_result<T, S>(&'config self, keys: S) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        self.get_result_internal(&keys.try_into()?)
    }

    pub fn get_result_internal<T>(
        &'config self,
        keys: &CompoundKey,
    ) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
    {
        common::get_result_internal(self.roots.iter().filter_map(|def| def.node), keys)
    }

    pub fn try_convert_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_cloned().and_then(|node| node.try_convert_into())
    }

    pub fn merge_cloned(mut self) -> Result<ConfigurationNode, ConfigurationError> {
        common::merge_cloned(self.roots.drain(..).filter_map(|def| def.node))
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
