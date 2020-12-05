use async_trait::async_trait;
use configuration_rs::{
    builder::{AsyncConfigurationBuilder, ConfigurationBuilder},
    error::ConfigurationError,
    format,
    format::Format,
    source::{AsyncSource, FileSource, InMemorySource, Source},
};
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::test]
async fn test_empty_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();

    let configuration = builder.build().await.unwrap();

    assert_eq!(0, configuration.infos().count());
}

#[tokio::test]
async fn test_adding_sync_source_to_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(r#"{ "value" : 1 }"#),
        format::json(),
    );

    let configuration = builder.build().await.unwrap();

    assert_eq!(1, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![format::json().describe()]));
    assert!(configuration
        .infos()
        .map(|i| i.source())
        .eq(vec![InMemorySource::default().describe()]));
}

#[tokio::test]
async fn test_adding_more_sync_sources_to_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(r#"{ "value" : 1 }"#),
        format::json(),
    );
    builder.add(
        InMemorySource::from_string_slice(r#"{ "value" : 2 }"#),
        format::json(),
    );

    let json_desc = format::json().describe();
    let inmem_desc = InMemorySource::default().describe();

    let configuration = builder.build().await.unwrap();
    assert_eq!(2, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![&json_desc, &json_desc]));
    assert!(configuration
        .infos()
        .map(|i| i.source())
        .eq(vec![&inmem_desc, &inmem_desc]));
}

#[tokio::test]
async fn test_adding_async_source_to_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();

    let path: PathBuf = ["tests", "files", "config1.json5"].iter().collect();
    builder.add_async(TestAsyncFileSource::from_path(path), format::json5());

    let configuration = builder.build().await.unwrap();

    assert_eq!(1, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![format::json5().describe()]));
}

#[tokio::test]
async fn test_adding_more_asyncs_sources_to_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();

    let path1: PathBuf = ["tests", "files", "config1.json5"].iter().collect();
    builder.add_async(TestAsyncFileSource::from_path(path1), format::json5());

    let path2: PathBuf = ["tests", "files", "config1.json"].iter().collect();
    builder.add_async(TestAsyncFileSource::from_path(path2), format::json());

    let configuration = builder.build().await.unwrap();

    let json_desc = format::json().describe();
    let json5_desc = format::json5().describe();

    assert_eq!(2, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![&json5_desc, &json_desc]));
}

#[tokio::test]
async fn test_mixing_sync_and_async_in_async_builder() {
    let mut builder = AsyncConfigurationBuilder::default();

    let path: PathBuf = ["tests", "files", "config1.json5"].iter().collect();
    builder.add_async(TestAsyncFileSource::from_path(path), format::json5());

    let path: PathBuf = ["tests", "files", "config1.json"].iter().collect();
    builder.add(FileSource::from_path(path), format::json());

    let configuration = builder.build().await.unwrap();

    let json_desc = format::json().describe();
    let json5_desc = format::json5().describe();

    assert_eq!(2, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![&json5_desc, &json_desc]));
}

#[tokio::test]
async fn test_sync_to_async_builder() {
    let mut builder = ConfigurationBuilder::default();

    let path: PathBuf = ["tests", "files", "config1.json"].iter().collect();
    builder.add(FileSource::from_path(path), format::json());

    let path: PathBuf = ["tests", "files", "config1.json5"].iter().collect();
    let mut builder = builder.add_async(TestAsyncFileSource::from_path(path), format::json5());

    let configuration = builder.build().await.unwrap();

    let json_desc = format::json().describe();
    let json5_desc = format::json5().describe();

    assert_eq!(2, configuration.infos().count());
    assert!(configuration
        .infos()
        .map(|i| i.format())
        .eq(vec![&json_desc, &json5_desc]));
}

// ----------- Test implementations --------------- //

pub struct TestAsyncFileSource {
    path: PathBuf,
}

impl TestAsyncFileSource {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Self {
        TestAsyncFileSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl AsyncSource for TestAsyncFileSource {
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
