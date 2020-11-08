use super::Source;
use crate::error::ConfigurationError;

pub struct InMemorySource {
    value: Vec<u8>,
}

impl InMemorySource {
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
