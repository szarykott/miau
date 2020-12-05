use configuration_rs::{
    builder::ConfigurationBuilder, error::ErrorCode, format::Json, source::InMemorySource,
};
use serde::Deserialize;

static TEST_JSON: &'static str = r#"
{
    "map": {
        "entry": {
            "value1": true,
            "value2": {
                "array" : [1,2],
                "value3": "a"
            }
        }
    }
}"#;

// -------------- Happy path tests ------------------------- //

#[derive(Debug, Deserialize)]
struct Config {
    array: Vec<i32>,
    value3: String,
    optional: Option<i32>,
}

#[test]
fn test_basic_lens() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(TEST_JSON.trim()),
        Json::default(),
    );

    let configuration = builder.build().unwrap();
    let lens = configuration.lens().try_lens("map:entry").unwrap();

    assert_eq!(Some(true), lens.get("value1"));
    assert_eq!(Some("true"), lens.get("value1"));
    assert_eq!(Some(1), lens.get("value2:array:[0]"));
    assert!(true)
}

#[test]
fn test_double_lens() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(TEST_JSON.trim()),
        Json::default(),
    );

    let configuration = builder.build().unwrap();
    let lens = configuration.lens().try_lens("map:entry").unwrap();
    let inner_lens = lens.try_lens("value2").unwrap();

    assert_eq!(Some(true), lens.get("value1"));
    assert_eq!(Some(1), lens.get("value2:array:[0]"));
    assert_eq!(Some(2), lens.get("value2:array:[1]"));

    assert_eq!(Some(1), inner_lens.get("array:[0]"));
    assert_eq!(Some(2), inner_lens.get("array:[1]"));
    assert_eq!(Some("a"), inner_lens.get("value3"));
}

#[test]
fn test_lensing_with_multiple_configs() {
    let json1 = r#"
    {
        "map" : {
            "value" : 1,
            "array" : [1, 2, 3]
        },
        "array" : [1]
    }
    "#
    .trim();

    let json2 = r#"
    {
        "map" : {
            "value" : 2,
            "array" : [3]
        },
        "array" : [22, 23, 24, 25]
    }
    "#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_string_slice(json1), Json::default());
    builder.add(InMemorySource::from_string_slice(json2), Json::default());

    let configuration = builder.build().unwrap();
    let lens = configuration.lens().try_lens("map").unwrap();

    assert_eq!(Some(3), lens.get("array:[0]"));
    assert_eq!(Some(2), lens.get("array:[1]"));
    assert_eq!(Some(3), lens.get("array:[2]"));
    assert_eq!(Some("2".to_string()), lens.get("value")); // TODO: mention this to string in docs
    assert_eq!(None, lens.get::<i32, &str>("array:[3]"));
}

// ----------------- Strongly typed tests ------------------------- //

#[test]
fn test_lens_into_struct() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(TEST_JSON.trim()),
        Json::default(),
    );

    let configuration = builder.build().unwrap();
    let lens = configuration.lens().try_lens("map:entry:value2").unwrap();

    let config = lens.try_convert_into::<Config>().unwrap();

    assert!(vec![1, 2].iter().eq(config.array.iter()));
    assert_eq!("a", config.value3);
    assert_eq!(None, config.optional);
    assert!(true)
}

// --------------- Failure tests ---------------------- //

#[test]
fn test_lens_key_unparsable() {
    let mut builder = ConfigurationBuilder::default();

    let configuration = builder.build().unwrap();
    let error = configuration.lens().try_lens("drooids:[a]").unwrap_err();

    assert!(std::matches!(error.get_code(), ErrorCode::ParsingError(..)));
    assert!(error.to_string().contains("drooids:[a]"));
}
