use miau::{
    builder::ConfigurationBuilder, format::Json, provider::EnvironmentProvider,
    source::InMemorySource,
};
use serde_json::json;
use std::env;

#[test]
fn test_environment_source_no_prefix() {
    env::set_var("my_awesome_key", "my_awesome_value");
    env::set_var("notmy_awesome_key", "notmy_awesome_value");

    let mut builder = ConfigurationBuilder::default();
    builder.add_provider(EnvironmentProvider::new());

    let configuration = builder.build().unwrap();

    assert_eq!(
        Some("my_awesome_value"),
        configuration.get("my_awesome_key")
    );

    assert_eq!(
        Some("notmy_awesome_value"),
        configuration.get("notmy_awesome_key")
    );
}

#[test]
fn test_environment_source_compound_key() {
    env::set_var("t2_key:another_key", "my_awesome_value");

    let mut builder = ConfigurationBuilder::default();
    builder.add(
        InMemorySource::from_string_slice(
            json!({
                "t2_key" : {
                    "first_key" : "first_value",
                    "another_key" : "another_value"
                }
            })
            .to_string()
            .as_ref(),
        ),
        Json::default(),
    );
    builder.add_provider(EnvironmentProvider::new());

    let configuration = builder.build().unwrap();

    assert_eq!(Some("first_value"), configuration.get("t2_key:first_key"));
    assert_eq!(
        Some("my_awesome_value"),
        configuration.get("t2_key:another_key")
    );
}

#[test]
fn test_environment_source_with_prefix() {
    env::set_var("my_t3_awesome_key", "my_awesome_value");
    env::set_var("notmy_t3_awesome_key", "notmy_awesome_value");

    let mut builder = ConfigurationBuilder::default();
    builder.add_provider(EnvironmentProvider::with_prefix("my"));

    let configuration = builder.build().unwrap();

    assert_eq!(
        Some("my_awesome_value"),
        configuration.get("my_t3_awesome_key")
    );

    assert_eq!(
        None,
        configuration.get::<&str, &str>("notmy_t3_awesome_key")
    );
}
