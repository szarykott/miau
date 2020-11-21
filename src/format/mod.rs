use crate::{configuration::Configuration, error::ConfigurationError};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "serde_toml")]
mod serde_toml;

#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;
#[cfg(feature = "serde_toml")]
pub use serde_toml::Toml;

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
