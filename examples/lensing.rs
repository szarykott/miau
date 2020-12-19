use core::panic;
use miau::{
    builder::ConfigurationBuilder, configuration::ConfigurationRead, format, source::FileSource,
};
use std::collections::HashMap;

fn main() {
    let mut some_collection: HashMap<String, String> = HashMap::new();
    some_collection.insert("key".into(), "value".into());

    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add(
            FileSource::from_path("./files/config.json"), // specify source first
            format::json(), // predefined formats can be specified with format::* helper methods
        )
        .add_provider(some_collection)
        .build(); // only now all values will be fetched

    let configuration = match result {
        Ok(configuration) => configuration,
        Err(e) => panic!(
            "Please make sure you run `cargo run` from examples folder! {}",
            e.pretty_display()
        ),
    };

    // println!("{:#?}", configuration); // uncomment this line to see pretty printed debug display of configuration

    let lens = configuration.lens();
    let map_lens = lens.try_lens("map");
    let array_lens = lens.try_lens("array");

    match map_lens {
        Ok(lens) => {
            let word: Option<String> = lens.get("word");
            assert_eq!(Some("word".to_string()), word);
        }
        // lensing operation can fail if e.g key is unparsable
        Err(e) => panic!("Oh no! {}", e),
    };

    match array_lens {
        Ok(lens) => {
            let word: Option<String> = lens.get("[1]");
            assert_eq!(Some("2".to_string()), word);

            // lets convert it into strongly typed struct
            let stconfig = match lens.try_convert_into::<Vec<i64>>() {
                Ok(config) => config,
                Err(e) => panic!(
                    "You should not panic here in your app! {}",
                    e.pretty_display() // notice pretty_display here
                ),
            };

            assert_eq!(Some(&2), stconfig.get(1));
        }
        // lensing operation can fail if e.g key is unparsable
        Err(e) => panic!("Oh no! {}", e),
    };
}
