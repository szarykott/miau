use miau::{
    builder::ConfigurationBuilder,
    configuration::{ConfigurationRead, ConfigurationTree},
    format::Json,
    source::InMemorySource,
};
use serde_json::{from_str, json};

#[test]
fn test_configuration_as_configuration_source() {
    let mut builder1 = ConfigurationBuilder::default();
    builder1.add(
        InMemorySource::from_string_slice(json!({ "value" : 1 }).to_string().as_str()),
        Json::default(),
    );

    let configuration1 = builder1.build().unwrap();

    assert_eq!(Some(1), configuration1.get("value"));

    let mut builder2 = ConfigurationBuilder::default();
    builder2.add_provider(configuration1);

    let configuration2 = builder2.build().unwrap();

    assert_eq!(Some(1), configuration2.get("value"));
}

#[test]
fn test_configuration_node_as_configuration_source() {
    let node = from_str::<ConfigurationTree>(json!({ "value" : 1 }).to_string().as_ref()).unwrap();

    let mut builder = ConfigurationBuilder::default();
    builder.add_provider(node);

    let configuration = builder.build().unwrap();

    assert_eq!(Some(1), configuration.get("value"));
}
