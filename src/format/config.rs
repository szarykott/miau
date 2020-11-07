use crate::{
    configuration::{Configuration, SingularConfiguration},
    error::ConfigurationError,
    format::Provider,
};

impl Provider for Configuration {
    fn transform(&self, _input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }
}

impl Provider for SingularConfiguration {
    fn transform(&self, _input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        Ok(Configuration {
            roots: vec![self.root.clone()],
        })
    }
}
