use crate::{configuration::Configuration, error::ConfigurationError};

mod config;
mod json;
mod node;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

pub trait Provider {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError>;
}

impl<T> Provider for T
where
    T: Fn(Vec<u8>) -> Result<Configuration, ConfigurationError>,
{
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        self(input)
    }
}
