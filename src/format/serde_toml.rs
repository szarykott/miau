use crate::{
    configuration::Configuration,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

pub struct Toml {}

impl Toml {
    pub fn new() -> Self {
        Toml {}
    }
}

impl Default for Toml {
    fn default() -> Self {
        Toml::new()
    }
}

impl Format for Toml {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        toml::from_slice::<Configuration>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }
}
