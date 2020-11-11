use configuration_rs::{builder::ConfigurationBuilder, format::Json, source::InMemorySource};
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

    let config = configuration.try_into::<Config>().unwrap();

    assert!(vec![1, 2].iter().eq(config.array.iter()));
    assert_eq!("a", config.value3);
    assert_eq!(None, config.optional);
}
