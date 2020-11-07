use super::Source;
use crate::error::{ConfigurationError, ErrorCode};
use std::{collections::HashMap, env};

pub struct EnvironmentSource {
    prefix: Option<String>,
}

impl EnvironmentSource {
    pub fn new(prefix: Option<String>) -> Self {
        EnvironmentSource { prefix }
    }

    fn get(&self) -> Result<Vec<u8>, ConfigurationError> {
        let mut result: HashMap<String, String> = HashMap::new();
        let vars = env::vars();

        match self.prefix {
            Some(ref prefix) => push(&mut result, vars.filter(|(key, _)| key.starts_with(prefix))),
            None => push(&mut result, vars),
        }

        serde_json::to_vec(&result).map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}

fn push(result: &mut HashMap<String, String>, keys: impl Iterator<Item = (String, String)>) {
    for (key, value) in keys {
        result.insert(key, value);
    }
}

impl Source for EnvironmentSource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        self.get()
    }
}
