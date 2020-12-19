use super::Source;
use crate::error::ConfigurationError;
use std::default::Default;

/// Represents in memory configuration source.
pub struct InMemorySource {
    value: Vec<u8>,
}

impl InMemorySource {
    ///Creates new empty `InMemorySource`
    pub fn empty() -> Self {
        InMemorySource { value: Vec::new() }
    }

    ///Creates new `InMemorySource` from `&str`.
    ///
    ///# Example
    ///```rust
    ///use miau::builder::ConfigurationBuilder;
    ///use miau::source::InMemorySource;
    ///use miau::format::Json;
    ///
    ///let mut builder = ConfigurationBuilder::default();
    ///
    ///let c1 = r#"{"value" : 1}"#;
    ///
    ///builder.add(InMemorySource::from_string_slice(&c1), Json::new());
    ///let configuration = builder.build().unwrap(); // do not do it in you app
    ///```
    pub fn from_string_slice(string: &str) -> Self {
        InMemorySource {
            value: string.as_bytes().to_vec(),
        }
    }

    ///Creates new `InMemorySource` from `Vec<u8>`.
    pub fn from_bytes(input: Vec<u8>) -> Self {
        InMemorySource { value: input }
    }
}

impl Default for InMemorySource {
    fn default() -> Self {
        InMemorySource::empty()
    }
}

impl Source for InMemorySource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        Ok(self.value.clone())
    }

    fn describe(&self) -> String {
        "inmemory".into()
    }
}
