use miau::{
    builder::ConfigurationBuilder, configuration::ConfigurationRead, format, source::InMemorySource,
};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Config {
    value1: i32,
    value2: String,
    value3: Option<i32>,
    value4: bool,
}

#[test]
fn test_msgpack_format() {
    let config = Config {
        value1: 1,
        value2: "aha".into(),
        value3: None,
        value4: true,
    };

    let ser = rmp_serde::to_vec_named(&config).unwrap();

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_bytes(ser), format::msgpack());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value1"));
    assert_eq!(Some("aha"), configuration.get("value2"));
    assert_eq!(
        None,
        ConfigurationRead::<'_, &str, &str>::get(&configuration, "value3")
    );
    assert_eq!(Some(true), configuration.get("value4"));
    assert_eq!(
        None,
        ConfigurationRead::<'_, &str, &str>::get(&configuration, "value5")
    );
}
