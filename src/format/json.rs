use crate::{
    configuration::Configuration, de::ConfigurationDeserializer, error::SourceDeserializationError,
};
use std::default::Default;

pub struct JsonDeserializer {}

impl JsonDeserializer {
    pub fn new() -> Self {
        JsonDeserializer {}
    }
}

impl Default for JsonDeserializer {
    fn default() -> Self {
        JsonDeserializer::new()
    }
}

impl ConfigurationDeserializer for JsonDeserializer {
    fn deserialize(&self, input: String) -> Result<Configuration, SourceDeserializationError> {
        serde_json::from_str::<Configuration>(&input)
            .map_err(|e| SourceDeserializationError::SerdeError(e.to_string()))
    }
}
