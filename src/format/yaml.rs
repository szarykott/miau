use crate::{
    configuration::Node,
    error::{ConfigurationError, ErrorCode},
    format::Transformer,
};
use std::default::Default;

pub struct YamlDeserializer {}

impl YamlDeserializer {
    pub fn new() -> Self {
        YamlDeserializer {}
    }
}

impl Default for YamlDeserializer {
    fn default() -> Self {
        YamlDeserializer::new()
    }
}

impl Transformer for YamlDeserializer {
    fn transform(&self, input: String) -> Result<Node, ConfigurationError> {
        serde_yaml::from_str::<Node>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
