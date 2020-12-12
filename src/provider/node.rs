use crate::{
    configuration::{Configuration, ConfigurationInfo, ConfigurationTree},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for ConfigurationTree {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }

    fn describe(&self) -> ConfigurationInfo {
        ConfigurationInfo::new("other node", "unknown")
    }
}
