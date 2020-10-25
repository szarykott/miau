use crate::{configuration::Node, error::ConfigurationError};

mod config;
mod json;
mod node;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

// TODO: Rethink return type as Configuration needs to be merged to implement it
// TODO: Think about how to make Node private and return something else here
// TODO: Think about input being Vec<u8> for genericness
pub trait Transformer {
    fn transform(&self, input: String) -> Result<Node, ConfigurationError>;
}

impl<T> Transformer for T
where
    T: Fn(String) -> Result<Node, ConfigurationError>,
{
    fn transform(&self, input: String) -> Result<Node, ConfigurationError> {
        self(input)
    }
}
