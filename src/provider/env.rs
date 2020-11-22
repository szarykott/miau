use super::Provider;
use crate::{
    configuration::{merge, Configuration, ConfigurationNode, Key, Value},
    parsing,
};
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
        let vars = env::vars();

        let node = match self.prefix {
            Some(ref prefix) => push(vars.filter(|(key, _)| key.starts_with(prefix))),
            None => push(vars),
        };

        match node {
            Some(node) => Configuration { roots: vec![node] },
            None => Configuration { roots: vec![] },
        }
    }
}

fn push(keys: impl Iterator<Item = (String, String)>) -> Option<ConfigurationNode> {
    let mut trees: Vec<ConfigurationNode> = Vec::new();
    for (key, value) in keys {
        if let Ok(ckey) = parsing::str_to_key(key.as_ref()) {
            let all_map = ckey
                .iter()
                .all(|k| if let Key::Map(_) = k { true } else { false });

            if !all_map {
                continue;
            }

            trees.push(create_tree(ckey.iter().map(|k| k.unwrap_map()), value));
        }
    }

    let mut drain = trees.drain(..);
    match drain.next() {
        Some(node) => {
            if let Ok(final_node) = drain.try_fold(node, |f, s| merge(f, s)) {
                Some(final_node)
            } else {
                None
            }
        }
        None => None,
    }
}

fn create_tree(mut keys: impl Iterator<Item = String>, value: String) -> ConfigurationNode {
    match keys.next() {
        Some(key) => {
            let mut map = HashMap::new();
            map.insert(key, create_tree(keys, value));
            ConfigurationNode::Map(map)
        }
        None => ConfigurationNode::Value(Some(Value::String(value))),
    }
}

impl Provider for EnvironmentProvider {
    fn collect(&self) -> Result<Configuration, crate::error::ConfigurationError> {
        Ok(self.get())
    }
}
