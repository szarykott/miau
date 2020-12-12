use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `json` data format.
pub struct Json {}

impl Json {
    /// Creates new `Json` instance.
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
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        serde_json::from_slice::<ConfigurationTree>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "json".into()
    }
}
