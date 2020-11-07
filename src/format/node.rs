use crate::{
    configuration::{Configuration, Node},
    error::ConfigurationError,
    format::Provider,
};

impl Provider for Node {
    fn transform(&self, _input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }
}
