use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `Json5` data format.
pub struct Json5 {}

impl Json5 {
    /// Creates new `Json5` instance.
    pub fn new() -> Self {
        Json5 {}
    }
}

impl Default for Json5 {
    fn default() -> Self {
        Json5::new()
    }
}

impl Format for Json5 {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        json5::from_str::<ConfigurationTree>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "json5".into()
    }
}
