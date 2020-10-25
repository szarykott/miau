use crate::{configuration::Configuration, error::ConfigurationError};

mod config;
mod json;
mod node;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

// TODO: Think about how to make Node private
// TODO: Think about input being Vec<u8> for genericness
pub trait Transformer {
    fn transform(&self, input: String) -> Result<Configuration, ConfigurationError>;
}

impl<T> Transformer for T
where
    T: Fn(String) -> Result<Configuration, ConfigurationError>,
{
    fn transform(&self, input: String) -> Result<Configuration, ConfigurationError> {
        self(input)
    }
}
