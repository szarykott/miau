use crate::{
    de::ConfigurationDeserializer,
    configuration::Configuration,
    error::SourceDeserializationError
}; 
use std::{
    default::Default
};

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
    fn deserialize(&self, input: String) -> Result<Configuration, SourceDeserializationError> {
        serde_yaml::from_str::<Configuration>(&input)
            .map_err(|e| SourceDeserializationError::SerdeError(e.to_string()))
    }
}