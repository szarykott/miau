use super::{CompoundKey, Node, SingularConfiguration, Value};
use crate::error::ConfigurationError;
use serde::de::DeserializeOwned;
use std::convert::{From, TryFrom, TryInto};

//TODO: Can Lens support plural configuration in any way?
/// Provides lensing capabilities to a configuration reader.
/// Allows scoping into configuration section of choice for read only access.
pub struct Lens<'config> {
    handle: &'config Node,
}

impl<'config> Lens<'config> {
    pub(crate) fn new(node: &'config Node) -> Self {
        Lens { handle: node }
    }

    pub fn try_lens<S>(&self, keys: S) -> Result<Self, ConfigurationError>
    where
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        Ok(Lens {
            handle: self.handle.descend_many(&keys.try_into()?)?,
        })
    }

    pub fn get<T, S>(&'config self, keys: S) -> Option<T>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        self.handle.get_option(&keys.try_into().ok()?)
    }

    pub fn get_result<T, S>(&'config self, keys: S) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        self.handle.get_result(&keys.try_into()?)
    }

    pub fn try_into<T: DeserializeOwned>(&self) -> Result<T, ConfigurationError> {
        self.handle.try_into()
    }
}

impl<'config> From<&'config SingularConfiguration> for Lens<'config> {
    fn from(config: &'config SingularConfiguration) -> Self {
        Lens::new(&config.root)
    }
}
