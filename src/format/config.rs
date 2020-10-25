use crate::{
    configuration::{Configuration, SingularConfiguration},
    error::ConfigurationError,
    format::Transformer,
};

impl Transformer for Configuration {
    fn transform(&self, _input: String) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }
}

impl Transformer for SingularConfiguration {
    fn transform(&self, _input: String) -> Result<Configuration, ConfigurationError> {
        Ok(Configuration {
            roots: vec![self.root.clone()],
        })
    }
}
