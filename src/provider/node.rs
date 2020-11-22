use crate::{
    configuration::{Configuration, ConfigurationInfo, ConfigurationNode},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for ConfigurationNode {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }

    fn describe(&self) -> ConfigurationInfo {
        ConfigurationInfo::new("other node", "unknown")
    }
}
