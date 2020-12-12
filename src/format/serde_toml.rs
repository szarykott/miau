use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `toml` data format.
pub struct Toml {}

impl Toml {
    /// Creates new `Toml` instance.
    pub fn new() -> Self {
        Toml {}
    }
}

impl Default for Toml {
    fn default() -> Self {
        Toml::new()
    }
}

impl Format for Toml {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        toml::from_slice::<ConfigurationTree>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "toml".into()
    }
}
