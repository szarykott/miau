use super::Source;
use crate::error::ConfigurationError;

pub(crate) struct DummySource;

impl Source for DummySource {
    fn collect(&self) -> Result<String, ConfigurationError> {
        Ok(String::default())
    }
}
