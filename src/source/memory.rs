use super::Source;
use crate::error::ConfigurationError;
use std::string::FromUtf8Error;

pub struct InMemorySource {
    value: Vec<u8>,
}

impl InMemorySource {
    pub fn from_utf8_slice(slice: &[u8]) -> Result<Self, FromUtf8Error> {
        let buffer: Vec<u8> = slice.iter().cloned().collect();
        Ok(InMemorySource { value: buffer })
    }

    pub fn from_str(string: &str) -> Self {
        InMemorySource {
            value: string.as_bytes().iter().cloned().collect(),
        }
    }
}

impl Source for InMemorySource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        Ok(self.value.clone())
    }
}
