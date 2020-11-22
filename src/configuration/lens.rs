use super::{common, CompoundKey, ConfigurationNode, Value};
use crate::error::ConfigurationError;
use serde::de::DeserializeOwned;
use std::convert::{From, TryFrom, TryInto};

/// Provides lensing capabilities to a configuration reader.
/// Allows scoping into configuration section of choice for read only access.
pub struct Lens<'config> {
    handle: Vec<Option<&'config ConfigurationNode>>,
}

//TODO: Make some functions common with Configuration
impl<'config> Lens<'config> {
    pub(crate) fn new(node: &'config ConfigurationNode) -> Self {
        Lens {
            handle: vec![Some(node)],
        }
    }

    pub fn try_lens<S>(&self, keys: S) -> Result<Self, ConfigurationError>
    where
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        let keys = keys.try_into()?;
        let new_handle = self
            .handle
            .iter()
            .map(|root| match root {
                Some(node) => match node.descend_many(&keys) {
                    Ok(nhdl) => Some(nhdl),
                    Err(_) => None,
                },
                None => None,
            })
            .collect();
        Ok(Lens { handle: new_handle })
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
        common::get_result_option_internal(&self.handle, keys)
    }

    pub fn try_convert_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        self.merge_cloned().and_then(|node| node.try_convert_into())
    }

    pub fn merge_cloned(mut self) -> Result<ConfigurationNode, ConfigurationError> {
        common::merge_cloned(self.handle.drain(..).filter_map(|e| e))
    }
}

impl<'config> From<&'config ConfigurationNode> for Lens<'config> {
    fn from(config: &'config ConfigurationNode) -> Self {
        Lens::new(&config)
    }
}
