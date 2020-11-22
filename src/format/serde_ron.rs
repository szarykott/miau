use crate::{
    configuration::ConfigurationNode,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Experimental RON deserializer.
/// Might require hacks to work.
pub struct Ron {}

impl Ron {
    pub fn new() -> Self {
        Ron {}
    }
}

impl Default for Ron {
    fn default() -> Self {
        Ron::new()
    }
}

impl Format for Ron {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        ron::from_str::<ConfigurationNode>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "ron".into()
    }
}
