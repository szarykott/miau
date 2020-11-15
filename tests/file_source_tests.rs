use configuration_rs::{
    builder::ConfigurationBuilder,
    error::ErrorCode,
    format::{Json, Yaml},
    source::FileSource,
};
use std::path::PathBuf;

#[test]
fn test_file_source_json() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.json"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), Json::default());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_yaml() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.yaml"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), Yaml::default());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_missing_file_source() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "not_present.json"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path.clone()), Json::default());

    let error = builder.build().unwrap_err();

    println!("{}", error);

    assert!(std::matches!(error.get_code(), ErrorCode::IoError(..)));
    let error_string = error.to_string();
    assert!(error_string.contains(&path.display().to_string()))
}
