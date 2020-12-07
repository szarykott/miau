use miau::{builder::ConfigurationBuilder, error::ErrorCode, format, source::FileSource};
use std::path::PathBuf;

#[test]
fn test_file_source_json() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.json"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::json());

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
    builder.add(FileSource::from_path(path), format::yaml());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_toml() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.toml"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::toml());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_json5() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.json5"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::json5());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_ron() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.ron"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::ron());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_ini() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.ini"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::ini());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some(true), configuration.get("value2"));
    assert_eq!(None, configuration.get::<&str, &str>("value3"));
    assert_eq!(Some("aha"), configuration.get("value4"));
    assert_eq!(None, configuration.get::<&str, &str>("value5"));
}

#[test]
fn test_file_source_xml() {
    // done like this for correct execution on different OS
    let path: PathBuf = ["tests", "files", "config1.xml"].iter().collect();

    let mut builder = ConfigurationBuilder::default();
    builder.add(FileSource::from_path(path), format::xml());

    let configuration = builder.build().unwrap();

    println!("{:#?}", configuration);

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
    builder.add(FileSource::from_path(path.clone()), format::json());

    let error = builder.build().unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::IoError(..)));
    let error_string = error.to_string();
    assert!(error_string.contains(&path.display().to_string()))
}
