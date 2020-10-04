use crate::{
    configuration::ConfigurationRoot,
    format::ConfigurationDeserializer,
    error::{ConfigurationError, ErrorCode},
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

impl ConfigurationDeserializer for YamlDeserializer {
    fn deserialize(&self, input: String) -> Result<ConfigurationRoot, ConfigurationError> {
        serde_yaml::from_str::<ConfigurationRoot>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
