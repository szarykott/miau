use crate::{
    configuration::ConfigurationNode,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct Ini {}

impl Ini {
    pub fn new() -> Self {
        Ini {}
    }
}

impl Default for Ini {
    fn default() -> Self {
        Ini::new()
    }
}

impl Format for Ini {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        serde_ini::from_str::<ConfigurationNode>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "ini".into()
    }
}
