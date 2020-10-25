use crate::{
    configuration::Node,
    error::{ConfigurationError, ErrorCode},
    format::Transformer,
};
use std::default::Default;

pub struct JsonDeserializer {}

impl JsonDeserializer {
    pub fn new() -> Self {
        JsonDeserializer {}
    }
}

impl Default for JsonDeserializer {
    fn default() -> Self {
        JsonDeserializer::new()
    }
}

impl Transformer for JsonDeserializer {
    fn transform(&self, input: String) -> Result<Node, ConfigurationError> {
        serde_json::from_str::<Node>(&input)
            .map_err(|e| ErrorCode::SerdeError(e.to_string()).into())
    }
}
