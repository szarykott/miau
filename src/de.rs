use crate::{configuration::Configuration, error::ConfigurationError};

pub trait ConfigurationDeserializer {
    fn deserialize(&self, input: String) -> Result<Configuration, ConfigurationError>;
}

impl<T> ConfigurationDeserializer for T
where
    T: Fn(String) -> Result<Configuration, ConfigurationError>,
{
    fn deserialize(&self, input: String) -> Result<Configuration, ConfigurationError> {
        self(input)
    }
}
