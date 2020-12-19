//! # Miau
//!
//! ## What
//! **Async aware and extensible layered configuration system for Rust**
//! `Miau` allows you to gather configuration from many sources, including, but not limited to:
//! * files
//! * environment
//! * in memory
//!
//! It is built around `Serde` that does heavy lifitng part of transforming data formats into other ones. It is battle tested and perfromant.
//! `Miau` utilizes its capabilities to the fullest!
//!
//! `Miau`'s data model allows to layer configuration trees in order of choice and convert them into Rust structs
//! or use as-is retrieving configuration values directy from trees on the go.
//! Furthermore, it allows scoping into configuration sections using dead simple DSL keeping the same capabilities.
//!
//! It is built with async and extensibility in mind. Thanks to `async_trait` crate it is possible to define asynchronous configuration sources.
//!
//! ## How
//!
//! Building configuration is simple. Library defines two builders (for blocking and non-blocking sources also known as synchronous and asynchronous)
//! that accept sources and formats. It is lazy - no data will be fetched until builder is built.
//!
//! A simple example underneath.
//!```rust
//!use miau::{
//!    builder::ConfigurationBuilder, configuration::ConfigurationRead, format, format::Json5,
//!    provider::EnvironmentProvider, source::{FileSource, InMemorySource},
//!};
//!use std::collections::HashMap;
//!use std::env;
//!
//!fn main() {
//!    let mut some_collection: HashMap<String, String> = HashMap::new();
//!    some_collection.insert("key".into(), "value".into());
//!
//!    let mut builder = ConfigurationBuilder::default();
//!
//!    let result = builder
//!        .add_provider(EnvironmentProvider::with_prefix("ASDFA"))
//!        .add(
//!            InMemorySource::from_string_slice(r#"{"key" : "value"}"#),
//!            format::json())
//!        .add_provider(some_collection)
//!        .build();
//!
//!    let configuration = match result {
//!        Ok(configuration) => configuration,
//!        Err(e) => panic!(
//!            "You should handle it more gracefull in your app! {}",
//!            e.pretty_display()
//!        ),
//!    };
//!
//!    // `get` method is defined in ConfigurationRead trait. It has to be in scope!
//!    let from_map_then_array: Option<i32> = configuration.get("map:[1]");
//!}
//!```
//!
//! **Refer to examples inside the source code repository to learn how to construct configuration and define your own sources.**
//!
//!## Why
//!
//!While writing his own applications in Rust author of this library noticed that existing libraries to create layered configuration are either unmaintained, lack support for `async` or are in other ways not extensible enough.
//!
//!Goal of this library is to provide core functionality of creating layered configuration that is not likely to change often and can be extended via traits. It is not meant for `Miau` to become heavy or polutted with optional dependencies needed for specialized use cases.
//!
//!That is why its only heavy dependency is `serde` and it only defines `Sources` and `Providers` that can be implemented using standard library. Only most popular formats are part of `Miau` and even they are all feature flagged. This is also why no `async` trait is implemented - there are multiple heavy executors. For the same reason no HTTP source is included - HTTP libraries are numerous.
//!
//!Implementing support for aforementioned utilities should be done in separate crates (which is possible thanks to public traits).
//!
//!## Feature flags
//!
//!By default no feature flag is enabled.
//!
//!* `ini` - activates support for Ini format
//!* `json` - activates support for Json format
//!* `msgpack` - activates support for Message Pack format
//!* `serde_json5` - activates support for Json5 format
//!* `serde_toml` - activates support for Toml format
//!* `yaml` - activates support for Yaml format
//!* `all` - activates all other feature flags

#![deny(unsafe_code)]
#![deny(missing_docs)]
/// Configuration builders
pub mod builder;
/// Actual configuration
#[macro_use]
pub mod configuration;
mod de;
/// All configuration errors
pub mod error;
/// Configuration formats
pub mod format;
mod parsing;
/// Configuration providers
pub mod provider;
/// Configuration sources
pub mod source;
