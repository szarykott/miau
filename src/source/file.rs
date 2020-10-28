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
        let mut f = File::open(&self.path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
