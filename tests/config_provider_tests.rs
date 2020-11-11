use configuration_rs::{
    builder::ConfigurationBuilder, format::JsonDeserializer, source::InMemorySource,
};
use serde_json::json;

#[test]
fn test_configuration_as_configuration_source() {
    let mut builder1 = ConfigurationBuilder::default();
    builder1.add(
        InMemorySource::from_str(json!({ "value" : 1 }).to_string().as_str()),
        JsonDeserializer::default(),
    );

    let configuration1 = builder1.build().unwrap();

    assert_eq!(Some(1), configuration1.get_option("value"));

    let mut builder2 = ConfigurationBuilder::default();
    builder2.add_provider(configuration1);

    let configuration2 = builder2.build().unwrap();

    assert_eq!(Some(1), configuration2.get_option("value"));
}
