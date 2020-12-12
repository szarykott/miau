mod file;
mod memory;

use crate::error::ConfigurationError;
use async_trait::async_trait;

pub use file::FileSource;
pub use memory::InMemorySource;

/// Represents synchronous (blocking) config source.
pub trait Source {
    /// Synchronous function to fetch source data.
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError>;
    /// Description of the source.
    fn describe(&self) -> String;
}

#[async_trait]
/// Represents asynchronous (non-blocking) config source.
pub trait AsyncSource {
    /// Asynchronous function to fetch source data.
    ///
    /// It uses [`async_trait`](async_trait::async_trait) which is required to implement `AsyncSource`.
    async fn collect(&self) -> Result<Vec<u8>, ConfigurationError>;
    /// Description of the source.
    fn describe(&self) -> String;
}
