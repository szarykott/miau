use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `yaml` data format.
pub struct Yaml {}

impl Yaml {
    /// Creates new `Yaml` instance.
    pub fn new() -> Self {
        Yaml {}
    }
}

impl Default for Yaml {
    fn default() -> Self {
        Yaml::new()
    }
}

impl Format for Yaml {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        serde_yaml::from_slice::<ConfigurationTree>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "yaml".into()
    }
}
