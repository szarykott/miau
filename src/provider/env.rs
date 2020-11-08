use super::Provider;
use crate::configuration::Configuration;
use std::{collections::HashMap, convert::Into, env};

pub struct EnvironmentProvider {
    prefix: Option<String>,
}

impl EnvironmentProvider {
    pub fn new() -> Self {
        EnvironmentProvider { prefix: None }
    }

    pub fn with_prefix<T: Into<String>>(prefix: T) -> Self {
        EnvironmentProvider {
            prefix: Some(prefix.into()),
        }
    }

    fn get(&self) -> Configuration {
        let mut result: HashMap<String, String> = HashMap::new();
        let vars = env::vars();

        match self.prefix {
            Some(ref prefix) => push(&mut result, vars.filter(|(key, _)| key.starts_with(prefix))),
            None => push(&mut result, vars),
        }

        result.into()
    }
}

fn push(result: &mut HashMap<String, String>, keys: impl Iterator<Item = (String, String)>) {
    for (key, value) in keys {
        result.insert(key, value);
    }
}

impl Provider for EnvironmentProvider {
    fn collect(&self) -> Result<Configuration, crate::error::ConfigurationError> {
        Ok(self.get())
    }
}
