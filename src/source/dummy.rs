use super::Source;
use crate::error::ConfigurationError;

pub(crate) struct DummySource;

impl Source for DummySource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        Ok(Vec::default())
    }
}
