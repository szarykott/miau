use super::{CompoundKey, Node, Value};
use crate::error::ConfigurationError;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

/// Provides lensing capabilities to a configuration reader.
/// Allowes scoping into configuration section of choice for read only access.
pub struct Lens<'config> {
    handle: &'config Node,
}

impl<'config> Lens<'config> {
    pub fn lens(self, keys: &CompoundKey) -> Result<Self, ConfigurationError> {
        let mut next = self.handle;
        for key in keys.iter() {
            next = next.descend(key)?;
        }

        Ok(Lens { handle: next })
    }

    pub fn get_option<T>(&'config self, keys: &CompoundKey) -> Option<T>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
    {
        self.handle.get_option(keys)
    }

    pub fn get_result<T>(&'config self, keys: &CompoundKey) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
    {
        self.handle.get_result(keys)
    }

    pub fn try_into<'de, T: DeserializeOwned>(&self) -> Result<T, ConfigurationError> {
        self.handle.try_into()
    }
}
