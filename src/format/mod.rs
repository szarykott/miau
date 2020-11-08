use crate::{configuration::Configuration, error::ConfigurationError};

mod json;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

pub trait Transformer {
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError>;
}

impl<T> Transformer for T
where
    T: Fn(Vec<u8>) -> Result<Configuration, ConfigurationError>,
{
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        self(input)
    }
}
