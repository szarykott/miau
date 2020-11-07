use crate::{
    configuration::Configuration,
    error::{ConfigurationError, ErrorCode},
    format::Provider,
};
use std::default::Default;

pub struct YamlDeserializer {}

impl YamlDeserializer {
    pub fn new() -> Self {
        YamlDeserializer {}
    }
}

impl Default for YamlDeserializer {
    fn default() -> Self {
        YamlDeserializer::new()
    }
}

impl Provider for YamlDeserializer {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        serde_yaml::from_slice::<Configuration>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
