use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `ron` data format.
///
/// It uses external deserializer and is only as good as it is.
pub struct Ron {}

impl Ron {
    /// Creates new `Ron` instance.
    pub fn new() -> Self {
        Ron {}
    }
}

impl Default for Ron {
    fn default() -> Self {
        Ron::new()
    }
}

impl Format for Ron {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        ron::from_str::<ConfigurationTree>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "ron".into()
    }
}
