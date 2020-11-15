use crate::{error::ConfigurationError, source::Source};
use std::{
    convert::AsRef,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct FileSource {
    path: PathBuf,
}

impl FileSource {
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
}
