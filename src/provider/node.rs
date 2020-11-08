use crate::{
    configuration::{Configuration, Node},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for Node {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }
}
