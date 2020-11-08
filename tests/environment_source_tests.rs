use configuration_rs::{builder::ConfigurationBuilder, provider::EnvironmentProvider};
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
        configuration.get_option("my_awesome_key")
    );

    assert_eq!(
        Some("notmy_awesome_value"),
        configuration.get_option("notmy_awesome_key")
    );
}

#[test]
fn test_environment_source_with_prefix() {
    env::set_var("my_awesome_key", "my_awesome_value");
    env::set_var("notmy_awesome_key", "notmy_awesome_value");

    let mut builder = ConfigurationBuilder::default();
    builder.add_provider(EnvironmentProvider::with_prefix("my"));

    let configuration = builder.build().unwrap();

    assert_eq!(
        Some("my_awesome_value"),
        configuration.get_option("my_awesome_key")
    );

    assert_eq!(
        None,
        configuration.get_option::<&str, &str>("notmy_awesome_key")
    );
}
