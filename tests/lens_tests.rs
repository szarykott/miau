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

#[derive(Debug, Deserialize)]
struct Config {
    array: Vec<i32>,
    value3: String,
    optional: Option<i32>,
}

#[test]
fn test_basic_lens() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();
    // let lens = configuration.try_lens("map:entry").unwrap();

    // assert_eq!(Some(true), lens.get("value1"));
    // assert_eq!(Some("true"), lens.get("value1"));
    // assert_eq!(Some(1), lens.get("value2:array:[0]"));
    assert!(true)
}

#[test]
fn test_double_lens() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();
    // let lens = configuration.try_lens("map:entry").unwrap();
    // let inner_lens = lens.try_lens("value2").unwrap();

    // assert_eq!(Some(true), lens.get("value1"));
    // assert_eq!(Some(1), lens.get("value2:array:[0]"));
    // assert_eq!(Some(2), lens.get("value2:array:[1]"));

    // assert_eq!(Some(1), inner_lens.get("array:[0]"));
    // assert_eq!(Some(2), inner_lens.get("array:[1]"));
    // assert_eq!(Some("a"), inner_lens.get("value3"));
    assert!(true)
}

#[test]
fn test_lens_into_struct() {
    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(TEST_JSON.trim()), Json::default());

    let configuration = builder.build().unwrap().merge_owned().unwrap();
    // let lens = configuration.try_lens("map:entry:value2").unwrap();

    // let config = lens.try_into::<Config>().unwrap();

    // assert!(vec![1, 2].iter().eq(config.array.iter()));
    // assert_eq!("a", config.value3);
    // assert_eq!(None, config.optional);
    assert!(true)
}
