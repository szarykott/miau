use crate::{
    configuration::{Configuration, ConfigurationNode},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for ConfigurationNode {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }
}
