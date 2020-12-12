use super::Provider;
use crate::{
    configuration::{merge, Configuration, ConfigurationInfo, ConfigurationTree, Key, Value},
    parsing,
};
use std::{collections::HashMap, convert::Into, default::Default, env};

/// Provides environmental variables as configuration.
pub struct EnvironmentProvider {
    prefix: Option<String>,
}

impl EnvironmentProvider {
    /// Creates new `EnvironmentProvider` that retrives all environmental variables.
    pub fn new() -> Self {
        EnvironmentProvider { prefix: None }
    }

    /// Creates new `EnvironmentProvider` that retrives environmental variables prefixed with `prefix`.
    pub fn with_prefix<T: Into<String>>(prefix: T) -> Self {
        EnvironmentProvider {
            prefix: Some(prefix.into()),
        }
    }

    fn get(&self) -> Configuration {
        let vars = env::vars();

        let node = match self.prefix {
            Some(ref prefix) => push(vars.filter(|(key, _)| key.starts_with(prefix))),
            None => push(vars),
        };

        match node {
            Some(node) => Configuration::new_singular(
                ConfigurationInfo::new("environment", "environment"),
                node,
            ),
            None => Configuration::new_empty(),
        }
    }
}

fn push(keys: impl Iterator<Item = (String, String)>) -> Option<ConfigurationTree> {
    let mut trees: Vec<ConfigurationTree> = Vec::new();
    for (key, value) in keys {
        if let Ok(ckey) = parsing::str_to_key(key.as_ref()) {
            let all_map = ckey.iter().all(|k| std::matches!(k, Key::Map(..)));

            if !all_map {
                continue;
            }

            trees.push(create_tree(ckey.iter().map(|k| k.unwrap_map()), value));
        }
    }

    let mut drain = trees.drain(..);
    match drain.next() {
        Some(node) => {
            if let Ok(final_node) = drain.try_fold(node, merge) {
                Some(final_node)
            } else {
                None
            }
        }
        None => None,
    }
}

fn create_tree(mut keys: impl Iterator<Item = String>, value: String) -> ConfigurationTree {
    match keys.next() {
        Some(key) => {
            let mut map = HashMap::new();
            map.insert(key, create_tree(keys, value));
            ConfigurationTree::Map(map)
        }
        None => ConfigurationTree::Value(Some(Value::String(value))),
    }
}

impl Default for EnvironmentProvider {
    fn default() -> Self {
        EnvironmentProvider::new()
    }
}

impl Provider for EnvironmentProvider {
    fn collect(&self) -> Result<Configuration, crate::error::ConfigurationError> {
        Ok(self.get())
    }

    fn describe(&self) -> ConfigurationInfo {
        ConfigurationInfo::new("environment", "environment")
    }
}
