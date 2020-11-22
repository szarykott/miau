use crate::{
    configuration::ConfigurationNode,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct Yaml {}

impl Yaml {
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
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError> {
        serde_yaml::from_slice::<ConfigurationNode>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "yaml".into()
    }
}
