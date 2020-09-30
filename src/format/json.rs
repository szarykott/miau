use crate::{
    configuration::Configuration,
    de::ConfigurationDeserializer,
    error::{ConfigurationError, ErrorCode},
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
    fn deserialize(&self, input: String) -> Result<Configuration, ConfigurationError> {
        serde_json::from_str::<Configuration>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
