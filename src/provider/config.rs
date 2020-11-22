use crate::{configuration::Configuration, error::ConfigurationError, provider::Provider};

impl Provider for Configuration {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }
}
