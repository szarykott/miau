use async_trait::async_trait;
use miau::{
    builder::{AsyncConfigurationBuilder, ConfigurationBuilder},
    error::ConfigurationError,
    format,
    format::Format,
    source::{AsyncSource, FileSource, InMemorySource, Source},
};
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncReadExt};

// --------------------- Let's implement async file source first ----- //

pub struct AsyncFileSource {
    path: PathBuf,
}

impl AsyncFileSource {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Self {
        AsyncFileSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

// Notice [async trait here]! It is required to implement the trait.
#[async_trait]
impl AsyncSource for AsyncFileSource {
    async fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        let mut buffer = Vec::new();

        let mut f = File::open(&self.path)
            .await
            .map_err(|e| -> ConfigurationError { e.into() })
            .map_err(|e| {
                e.enrich_with_context(format!("Failed to open file : {}", self.path.display()))
            })?;

        f.read_to_end(&mut buffer)
            .await
            .map_err(|e| -> ConfigurationError { e.into() })
            .map_err(|e| {
                e.enrich_with_context(format!("Failed to read file : {}", self.path.display()))
            })?;

        Ok(buffer)
    }

    fn describe(&self) -> String {
        std::fs::canonicalize(&self.path)
            .unwrap_or_else(|_| self.path.clone())
            .display()
            .to_string()
    }
}

// ---------------------- Actual example begins -------------------------------- //

// In real applications you should be able to use [tokio::main] or similar from other runtimes
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    runtime.block_on(async { async_main().await });
}

async fn async_main() {
    // Let's start with synchronous builder to demonstrate how to combine it with async source
    // It is
    let mut builder = ConfigurationBuilder::new();
}
