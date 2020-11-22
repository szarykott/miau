use configuration_rs::{
    builder::ConfigurationBuilder, error::ErrorCode, format::Json, source::InMemorySource,
};
use serde::Deserialize;

static TEST_JSON: &'static str = r#"
{
    "map": {
        "array1" : [1,23],
        "entry": {
            "value1": true,
            "value2": {
                "array" : [1,2],
                "value3": "a"
            },
            "value3": "sdada"
        }
    }
}"#;

#[test]
fn test_basic_singular_configuration() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    assert_eq!(Some(true), configuration.get("map:entry:value1"));
    assert_eq!(Some("true"), configuration.get("map:entry:value1"));
    assert_eq!(Some(1), configuration.get("map:entry:value2:array:[0]"));
}

static TEST_JSON_2: &'static str = r#"
{
    "array" : [1,2],
    "value3": "a"
}"#;

#[derive(Debug, Deserialize)]
struct Config {
    array: Vec<i32>,
    value3: String,
    optional: Option<i32>,
}

#[test]
fn test_singular_configuration_into_struct() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_str(TEST_JSON_2.trim()),
        Json::default(),
    );

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let config = configuration.try_convert_into::<Config>().unwrap();

    assert!(vec![1, 2].iter().eq(config.array.iter()));
    assert_eq!("a", config.value3);
    assert_eq!(None, config.optional);
}

#[test]
fn test_singular_configuration_wrong_type_conversion() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let error = configuration
        .get_result::<i32, &str>("map:entry:value3")
        .unwrap_err(); // value1 is string

    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->entry-->value3"))
}

#[test]
fn test_singluar_configuration_index_out_of_range() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let error = configuration
        .get_result::<i32, &str>("map:entry:value2:array:[66]")
        .unwrap_err();

    assert!(std::matches!(
        error.get_code(),
        ErrorCode::IndexOutOfRange(..)
    ));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->entry-->value2-->array-->[66]"))
}

#[test]
fn test_singluar_configuration_key_not_found() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let error = configuration
        .get_result::<i32, &str>("map:entry:value2:arrayy:[66]") // typo in array
        .unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::KeyNotFound(..)));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->entry-->value2-->arrayy"))
}

#[test]
fn test_singluar_configuration_descending_into_non_descendable() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let error = configuration
        .get_result::<i32, &str>("map:entry:value1:[66]") // trying to index into bool
        .unwrap_err();

    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongNodeType(..)
    ));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->entry-->value1-->[66]"))
}

#[test]
fn test_singular_configuration_key_and_node_mismatch_descending() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();

    let error = configuration
        .get_result::<i32, &str>("map:[1]")
        .unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::WrongKeyType(..)));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->[1]"));

    let error = configuration
        .get_result::<i32, &str>("map:array1:one")
        .unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::WrongKeyType(..)));
    let error_string = error.to_string();
    assert!(error_string.contains("map-->array1-->one"));
}
