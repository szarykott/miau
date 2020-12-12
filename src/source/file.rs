use crate::{error::ConfigurationError, source::Source};
use std::{
    convert::AsRef,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// Represents configuration file source.
pub struct FileSource {
    path: PathBuf,
}

impl FileSource {
    /// Constructs new instance of `FileSource` pointing to file at `path`.
    pub fn from_path<T: AsRef<Path>>(path: T) -> Self {
        FileSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for FileSource {
    fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        let mut buffer = Vec::new();

        let mut f = File::open(&self.path)
            .map_err(|e| -> ConfigurationError { e.into() })
            .map_err(|e| {
                e.enrich_with_context(format!("Failed to open file : {}", self.path.display()))
            })?;

        f.read_to_end(&mut buffer)
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
