use configuration_rs::{
    builder::ConfigurationBuilder,
    format::{JsonDeserializer, YamlDeserializer},
    source::FileSource,
};
use std::path::PathBuf;

#[test]
fn test_file_source_json() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.json"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), JsonDeserializer::default());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get_option("value1"));
    assert_eq!(Some(true), configuration.get_option("value2"));
    assert_eq!(None, configuration.get_option::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get_option("value4"));
    assert_eq!(None, configuration.get_option::<&str, &str>("value5"));
}

#[test]
fn test_file_source_yaml() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.yaml"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), YamlDeserializer::default());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get_option("value1"));
    assert_eq!(Some(true), configuration.get_option("value2"));
    assert_eq!(None, configuration.get_option::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get_option("value4"));
    assert_eq!(None, configuration.get_option::<&str, &str>("value5"));
}

#[test]
fn test_missing_file_source() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "not_present.json"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), JsonDeserializer::default());

    let configuration = builder.build();

    assert!(configuration.is_err());
}
