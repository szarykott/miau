use crate::{configuration::Configuration, error::ConfigurationError};

mod json;
mod yaml;

pub use json::Json;
pub use yaml::Yaml;

pub trait Format {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError>;
}

impl<T> Format for T
where
    T: Fn(Vec<u8>) -> Result<Configuration, ConfigurationError>,
{
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        self(input)
    }
}
