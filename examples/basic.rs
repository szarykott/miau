use core::panic;
use miau::{
    builder::ConfigurationBuilder, configuration::ConfigurationRead, format, format::Json5,
    provider::EnvironmentProvider, source::FileSource,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
struct Config {
    map: HashMap<String, String>,
    array: Vec<i64>,
}

/// Example below presents basic configuration usage
/// Run this example with --all-features flag for simplicity
fn main() {
    let mut some_collection: HashMap<String, String> = HashMap::new();
    some_collection.insert("key".into(), "value".into());

    env::set_var("ASDFA_VAR", "envvar"); // setting some stupid env var for presentation

    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add_provider(EnvironmentProvider::with_prefix("ASDFA")) // providers are special sources that are simpler to handle without source,format division
        .add(
            FileSource::from_path("./examples/files/config.json"), // specify source first
            format::json(), // predefined formats can be specified with format::* helper methods
        )
        .add(
            FileSource::from_path("./examples/files/config.json5"),
            Json5::default(), // structs implementing `Format` trait can also be used directly
        )
        .add_provider(some_collection)
        .build(); // only now all values will be fetched

    let configuration = match result {
        Ok(configuration) => configuration,
        Err(e) => panic!("Failed to create configuration : {}", e),
    };

    env::remove_var("ASDFA_VAR");

    // println!("{:#?}", configuration); // uncomment this line to see pretty printed debug display of configuration

    let from_collection: Option<String> = configuration.get("key");
    assert_eq!(Some("value".to_string()), from_collection);

    let from_env: Option<String> = configuration.get("ASDFA_VAR");
    assert_eq!(Some("envvar".to_string()), from_env);

    let from_json: Option<i32> = configuration.get("map:value"); // index into maps by using ':' between keys
    assert_eq!(Some(1), from_json);

    let from_json5: Option<bool> = configuration.get("map:boolean");
    assert_eq!(Some(false), from_json5); // notice json5 overwrites this value

    let from_array: Option<i32> = configuration.get("array:[1]"); // use [x] to mark you are indexing into array
    assert_eq!(Some(2), from_array);

    // and now lets create strongly typed configuration

    let stconfig = match configuration.try_convert_into::<Config>() {
        Ok(config) => config,
        Err(e) => panic!(
            "You should not panic here in your app! {}",
            e.pretty_display() // notice pretty_display here
        ),
    };

    assert_eq!(Some(&4), stconfig.array.get(3));
    assert!(stconfig.map.contains_key(&"word".to_string()));
}
