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
//! ## Why
//!
//! It is important to note that this crate means to be as lightweight as possible, thus it provides only those sources that can be created
//! using standard library. Other sources, for example networks sources might be implemented in separate crates.
//! Reason for it is simple - usually there is more than one library to do one thing, for instance issue HTTP request.
//! As it is often impossible to choose the best one, `Miau` chooses not to make this decision and delegates it to other crates or users.
//!
//! ## How
//!
//! Building configuration is simple. Library defines two builders (for blocking and non-blocking sources also known as synchronous and asynchronous)
//! that accept sources and formats. It is lazy - no data will be fetched until builder is built.
//!
//!
//! **Refer to examples inside the source code repository to learn how to construct configuration and define your own sources.**

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
