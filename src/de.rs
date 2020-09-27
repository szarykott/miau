use crate::{configuration::Configuration, error::SourceDeserializationError};

pub trait ConfigurationDeserializer {
    fn deserialize(&self, input: String) -> Result<Configuration, SourceDeserializationError>;
}

impl<T> ConfigurationDeserializer for T
where
    T: Fn(String) -> Result<Configuration, SourceDeserializationError>,
{
    fn deserialize(&self, input: String) -> Result<Configuration, SourceDeserializationError> {
        self(input)
    }
}
