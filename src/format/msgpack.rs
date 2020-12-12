use crate::{
    configuration::ConfigurationTree,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// Represents `message pack` data format.
///
/// It is vital that configurations serialized with this format are named i.e contain field names.
pub struct Msgpack {}

impl Msgpack {
    /// Creates new `Msgpack` instance.
    pub fn new() -> Self {
        Msgpack {}
    }
}

impl Default for Msgpack {
    fn default() -> Self {
        Msgpack::new()
    }
}

impl Format for Msgpack {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        rmp_serde::from_slice::<ConfigurationTree>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "message pack".into()
    }
}
