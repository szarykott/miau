use super::Source;
use crate::error::ConfigurationError;
use std::string::FromUtf8Error;

pub struct InMemorySource {
    value: String,
}

impl InMemorySource {
    pub fn from_utf8_slice(slice: &[u8]) -> Result<Self, FromUtf8Error> {
        let buffer: Vec<u8> = slice.iter().cloned().collect();
        let value = String::from_utf8(buffer)?;
        Ok(InMemorySource { value })
    }

    pub fn from_str(string: &str) -> Self {
        InMemorySource {
            value: string.to_string(),
        }
    }
}

impl Source for InMemorySource {
    fn collect(&self) -> Result<String, ConfigurationError> {
        Ok(self.value.clone())
    }
}
