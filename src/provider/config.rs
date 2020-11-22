use crate::{
    configuration::{Configuration, ConfigurationInfo},
    error::ConfigurationError,
    provider::Provider,
};

impl Provider for Configuration {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone())
    }

    fn describe(&self) -> ConfigurationInfo {
        let formats: Vec<&str> = self
            .roots
            .iter()
            .map(|def| def.info.format.as_str())
            .collect();
        ConfigurationInfo::new(
            "other configuration".into(),
            format!("multiple({})", (&formats).join(",")),
        )
    }
}
