use crate::{
    configuration::{Configuration, SingularConfiguration},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for Configuration {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }
}

impl Provider for SingularConfiguration {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(Configuration {
            roots: vec![self.root.clone()],
        })
    }
}
