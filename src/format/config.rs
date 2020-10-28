use crate::{
    configuration::{Configuration, SingularConfiguration},
    error::ConfigurationError,
    format::Transformer,
};

impl Transformer for Configuration {
    fn transform(&self, _input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }
}

impl Transformer for SingularConfiguration {
    fn transform(&self, _input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        Ok(Configuration {
            roots: vec![self.root.clone()],
        })
    }
}
