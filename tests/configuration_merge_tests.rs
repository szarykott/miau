use configuration_rs::{
    builder::ConfigurationBuilder, error::ErrorCode, format::Json, source::InMemorySource,
};
use rstest::rstest;
use serde::Deserialize;
use std::collections::HashMap;

#[rstest(
    json1,
    json2,
    exp,
    case(
        r#"{"array1" : [1,2,3,4]}"#,
        r#"{"array1" : [4,5]}"#,
        vec![4, 5, 3, 4]
    ),
    case(
        r#"{"array1" : [1,2]}"#,
        r#"{"array1" : [4,5,6]}"#,
        vec![4, 5, 6]
    ),
    case(
        r#"{"array1" : []}"#,
        r#"{"array1" : [4,5,6]}"#,
        vec![4, 5, 6]
    ),
    case(
        r#"{"array1" : [4,5,6]}"#,
        r#"{"array1" : []}"#,
        vec![4, 5, 6]
    )
)]

fn test_arrays_are_merged_when_substituted(json1: &str, json2: &str, exp: Vec<i32>) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_string_slice(json1.as_ref()),
        Json::new(),
    );
    builder.add(
        InMemorySource::from_string_slice(json2.as_ref()),
        Json::new(),
    );

    let confiuration = builder.build().unwrap();

    let mut result = confiuration
        .try_convert_into::<HashMap<String, Vec<i32>>>()
        .unwrap();

    assert_eq!(exp, result.remove("array1".into()).unwrap());
}

#[test]
fn test_maps_are_merged_simple() {
    #[derive(Deserialize, Debug)]
    struct Config {
        value1: i32,
        value2: f64,
    }

    let cfg1 = r#"{ "value1" : 1 }"#;
    let cfg2 = r#"{ "value2" : -1.1 }"#;

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_string_slice(cfg1), Json::new());
    builder.add(InMemorySource::from_string_slice(cfg2), Json::new());

    let confiuration = builder.build().unwrap();

    let result = confiuration.try_convert_into::<Config>().unwrap();

    assert_eq!(result.value1, 1);
    assert_eq!(result.value2, -1.1);
}

#[test]
fn test_maps_are_merged_nested() {
    #[derive(Deserialize, Debug)]
    struct Config {
        value1: ConfigInner,
        value2: f64,
    }

    #[derive(Debug, Deserialize)]
    struct ConfigInner {
        value1: i32,
    }

    let cfg1 = r#"{ 
        "value1" : {
            "value1" : 12
        } 
    }"#
    .trim();

    let cfg2 = r#"{ 
        "value2" : -1.1,
        "value1" : {
            "value1" : 13
        } 
    }"#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_string_slice(cfg1), Json::new());
    builder.add(InMemorySource::from_string_slice(cfg2), Json::new());

    let confiuration = builder.build().unwrap();

    let result = confiuration.try_convert_into::<Config>().unwrap();

    assert_eq!(result.value1.value1, 13);
    assert_eq!(result.value2, -1.1);
}

#[rstest(
    cfg1,
    cfg2,
    exp_key,
    exp_for,
    case(
        r#"{"this_is_key" : 1}"#,
        r#"{"this_is_key" : [1]}"#,
        "this_is_key",
        "array for value"
    ),
    case(
        r#"{"this_is_key" : {"key" : 1}}"#,
        r#"{"this_is_key" : [1]}"#,
        "this_is_key",
        "array for map"
    ),
    case(
        r#"{"this_is_key" : 1}"#,
        r#"{"this_is_key" : {"key" : 1}}"#,
        "this_is_key",
        "map for value"
    ),
    // one below is double to assert other key message
    case(
        r#"{"this_is_key" : { "key2" :  [1]} }"#,
        r#"{"this_is_key" : { "key2" :  {"key3" : 1}} }"#,
        "this_is_key-->key2",
        "map for array"
    ),
    case(
        r#"{"this_is_key" : [1]}"#,
        r#"{"this_is_key" : {"key" : 1}}"#,
        "this_is_key",
        "map for array"
    ),
    case(
        r#"{"this_is_key" : [1]}"#,
        r#"{"this_is_key" : 1}"#,
        "this_is_key",
        "value for array"
    ),
    case(
        r#"{"this_is_key":{"key":1}}"#,
        r#"{"this_is_key":1}"#,
        "this_is_key",
        "value for map"
    )
)]
fn test_node_merge_error_messages(cfg1: &str, cfg2: &str, exp_key: &str, exp_for: &str) {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_string_slice(cfg1), Json::new());
    builder.add(InMemorySource::from_string_slice(cfg2), Json::new());

    let confiuration = builder.build().unwrap();

    let error = confiuration.merge_owned().unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::BadNodeMerge(..)));
    let error_string = error.to_string();
    assert!(error_string.contains(exp_key));
    assert!(error_string.contains(exp_for));
}

#[test]
fn test_merge_empty_configuration_is_error() {
    let mut builder = ConfigurationBuilder::default();
    let configuration = builder.build().unwrap();

    let error = configuration.merge_owned().unwrap_err();

    println!("{}", error);

    assert!(std::matches!(
        error.get_code(),
        ErrorCode::EmptyConfiguration
    ));
    assert!(error.get_context().is_some());
    assert!(!error.get_context().unwrap().is_empty())
}
