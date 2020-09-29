use crate::{
    error::ConfigurationError,
    source::Source
};
use std::{
    fs::File,
    convert::AsRef,
    io::Read,
    path::{Path, PathBuf}
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
    fn collect(&self) -> Result<String, ConfigurationError> {
        let mut f = File::open(&self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
