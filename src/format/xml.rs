use crate::{
    configuration::Configuration,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct Xml {}

impl Xml {
    pub fn new() -> Self {
        Xml {}
    }
}

impl Default for Xml {
    fn default() -> Self {
        Xml::new()
    }
}

impl Format for Xml {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        serde_xml_rs::from_str::<Configuration>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }
}
