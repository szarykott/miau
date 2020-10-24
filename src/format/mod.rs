use crate::{configuration::Node, error::ConfigurationError};

mod config;
mod json;
mod node;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

pub trait ConfigurationDeserializer {
    fn deserialize(&self, input: String) -> Result<Node, ConfigurationError>;
}

impl<T> ConfigurationDeserializer for T
where
    T: Fn(String) -> Result<Node, ConfigurationError>,
{
    fn deserialize(&self, input: String) -> Result<Node, ConfigurationError> {
        self(input)
    }
}
