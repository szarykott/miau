mod file;
mod memory;

use crate::error::SourceCollectionError;
use async_trait::async_trait;

pub use file::FileSource;
pub use memory::InMemorySource;

/// Describes synchronous config source.
pub trait Source {
    /// Synchronous function to collect source into key value pairs.
    fn collect(&self) -> Result<String, SourceCollectionError>;
}

#[async_trait]
/// Describes asynchronous config source.
pub trait AsyncSource {
    /// Asynchronous function to collect source into key value pairs.
    async fn collect(&self) -> Result<String, SourceCollectionError>;
}