use super::Source;
use crate::error::ConfigurationError;
use std::default::Default;

pub struct InMemorySource {
    value: Vec<u8>,
}

impl InMemorySource {
    pub fn empty() -> Self {
        InMemorySource { value: Vec::new() }
    }

    pub fn from_string_slice(string: &str) -> Self {
        InMemorySource {
            value: string.as_bytes().to_vec(),
        }
    }

    pub fn from_bytes(input: Vec<u8>) -> Self {
        InMemorySource { value: input }
    }
}

impl Default for InMemorySource {
    fn default() -> Self {
        InMemorySource::empty()
    }
}

impl Source for InMemorySource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        Ok(self.value.clone())
    }

    fn describe(&self) -> String {
        "inmemory".into()
    }
}
