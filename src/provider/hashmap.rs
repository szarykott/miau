use super::Provider;
use crate::configuration::{Configuration, ConfigurationInfo, ConfigurationTree, Value};
use std::collections::HashMap;

impl Provider for HashMap<String, String> {
    fn collect(
        &self,
    ) -> Result<crate::configuration::Configuration, crate::error::ConfigurationError> {
        let mut result = HashMap::new();

        for (k, v) in self.iter() {
            result.insert(
                k.clone(),
                ConfigurationTree::Value(Some(Value::String(v.clone()))),
            );
        }

        Ok(Configuration::new_singular(
            self.describe(),
            ConfigurationTree::Map(result),
        ))
    }

    fn describe(&self) -> crate::configuration::ConfigurationInfo {
        ConfigurationInfo::new("hashmap", "hashmap")
    }
}
