use crate::{
    configuration::ConfigurationNode,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct Json {}

impl Json {
    pub fn new() -> Self {
        Json {}
    }
}

impl Default for Json {
    fn default() -> Self {
        Json::new()
    }
}

impl Format for Json {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError> {
        serde_json::from_slice::<ConfigurationNode>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "json".into()
    }
}
