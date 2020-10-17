use crate::{configuration::ConfigurationRoot, error::ConfigurationError};

mod json;
mod yaml;

pub use json::JsonDeserializer;
pub use yaml::YamlDeserializer;

pub trait ConfigurationDeserializer {
    fn deserialize(&self, input: String) -> Result<ConfigurationRoot, ConfigurationError>;
}

impl<T> ConfigurationDeserializer for T
where
    T: Fn(String) -> Result<ConfigurationRoot, ConfigurationError>,
{
    fn deserialize(&self, input: String) -> Result<ConfigurationRoot, ConfigurationError> {
        self(input)
    }
}
