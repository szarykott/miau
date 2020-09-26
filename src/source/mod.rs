mod memory;
mod file;

use async_trait::async_trait;
use crate::error::SourceCollectionError;

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
