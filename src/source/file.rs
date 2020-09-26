use super::Source;
use std::path::{PathBuf, Path};
use std::convert::AsRef;
use std::fs::File;
use crate::error::SourceCollectionError;
use std::io::Read;

struct FileSource {
    path : PathBuf,
    content : Option<String>
}

impl FileSource {
    pub fn from_path<T : AsRef<Path>>(path : T) -> Self {
        FileSource {
            path : path.as_ref().to_path_buf(),
            content : None
        }
    }
}

impl Source for FileSource {
    fn collect(&self) -> Result<String, SourceCollectionError> {
        let mut f = File::open(&self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}