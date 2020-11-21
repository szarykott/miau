use crate::{
    configuration::Configuration,
    error::{ConfigurationError, ErrorCode},
    format::Format,
};
use std::default::Default;

/// MessagePack format.
/// It is vital that configurations serialized with this format are named i.e contain field names.
pub struct Msgpack {}

impl Msgpack {
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
    fn transform(&self, input: Vec<u8>) -> Result<Configuration, ConfigurationError> {
        rmp_serde::from_slice::<Configuration>(&input)
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }
}
