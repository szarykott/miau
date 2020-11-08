mod utils;

use configuration_rs::{
    builder::ConfigurationBuilder, configuration::Configuration, format::JsonDeserializer, key,
    source::InMemorySource,
};
use rstest::rstest;

#[test]
fn test_arrays_are_subsituted_when_config_is_built() {
    let json1 = r#"{"array1" : [1,2,3,4]}"#;
    let json2 = r#"{"array1" : [5,6]}"#;
    let json3 = r#"{"array1" : [7]}"#;

    let mut builder = ConfigurationBuilder::default();

    builder
        .add(InMemorySource::from_str(json1), JsonDeserializer::new())
        .add(InMemorySource::from_str(json2), JsonDeserializer::new())
        .add(InMemorySource::from_str(json3), JsonDeserializer::new());

    let confiuration = builder.build().unwrap();

    assert_eq!(Some(7), confiuration.get_option("array1:[0]"));
    assert_eq!(Some(6), confiuration.get_option("array1:[1]"));
    assert_eq!(Some(3), confiuration.get_option("array1:[2]"));
    assert_eq!(Some(4), confiuration.get_option("array1:[3]"));
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : 2}"#, 2),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : 1}"#, 1),
    case(r#"{"value1" : false}"#, r#"{"value1" : 3}"#, 3),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : -4}"#, -4)
)]
fn test_type_to_integer_substitution(c1: &str, c2: &str, exp: isize) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(InMemorySource::from_str(c1), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2), JsonDeserializer::new());

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(Some(exp), result.get_option("value1"));
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : 2.1}"#, 2.1f64),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : 1.1}"#, 1.1f64),
    case(r#"{"value1" : false}"#, r#"{"value1" : 3.1}"#, 3.1f64),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : 4.1}"#, 4.1f64)
)]
fn test_type_to_float_substitution(c1: &str, c2: &str, exp: f64) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(InMemorySource::from_str(c1), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2), JsonDeserializer::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get_option("value1"));
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : false}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : false}"#, false)
)]
fn test_type_to_bool_substitution(c1: &str, c2: &str, exp: bool) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(InMemorySource::from_str(c1), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2), JsonDeserializer::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get_option("value1"));
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : false}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : "false"}"#, "false")
)]
fn test_type_to_string_substitution(c1: &str, c2: &str, exp: &str) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(InMemorySource::from_str(c1), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2), JsonDeserializer::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get_option("value1"));
}

#[test]
fn test_single_value_integer_config_build_json() {
    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add_provider(serde_json::from_str::<Configuration>("1").unwrap())
        .add_provider(serde_json::from_str::<Configuration>("2").unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2i32), result.get_option(""));
    assert_eq!(Some(2i32), result.get_option(key!()));
}

#[test]
fn test_single_map_entry_config_build_json() {
    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(r#"{"value" : 1}"#).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(r#"{"value" : 2}"#).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2), result.get_option("value"));
    assert_eq!(Some(2), result.get_option(key!("value")));
}

#[test]
fn test_single_map_entry_config_build_json_different_type() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : "2"}"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some("2"), result.get_option("value"));
    assert_eq!(Some(2), result.get_option("value"));
}

#[test]
fn test_two_different_map_entries_config_build_json() {
    let config_str_1 = r#"{"value1" : 1}"#;
    let config_str_2 = r#"{"value2" : 2}"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(1), result.get_option("value1"));
    assert_eq!(Some(2), result.get_option("value2"));
}

#[test]
fn test_single_array_entry_config_build_json() {
    let config_str_1 = r#"[1]"#;
    let config_str_2 = r#"[2]"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2), result.get_option("[0]"));
}

#[test]
fn test_complex_map_config_build_json() {
    let config_str_1 = r#"
    {
        "firstName": "John",
        "lastName": "Smith",
        "isAlive": true,
        "address": {
          "streetAddress": "21 2nd Street"
        },
        "phoneNumbers": [
          {
            "type": "home",
            "number": "212 555-1234"
          }
        ],
        "spouse": null
      }
    "#
    .trim();

    let config_str_2 = r#"
    {
        "firstName": "Andrew",
        "isAlive": false,
        "address": {
          "streetAddress": "Knowhere"
        },
        "phoneNumbers": [
          {
            "type": "work",
            "number": "212 555-1234"
          }
        ],
        "spouse": true
      }
    "#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some("Andrew"), result.get_option("firstName"));
    assert_eq!(Some("Smith"), result.get_option("lastName"));
    assert_eq!(Some(false), result.get_option("isAlive"));
    assert_eq!(Some("Knowhere"), result.get_option("address:streetAddress"));
    assert_eq!(Some("work"), result.get_option("phoneNumbers:[0]:type"));
    assert_eq!(Some(true), result.get_option("spouse"));
}

#[test]
fn test_array_of_structs_build_json() {
    let config_str_1 = r#"
    {
        "array" : [
            { "v" : 1, "k" : 11 },
            { "v" : 3, "k" : 33 }
        ]
    }
    "#
    .trim();

    let config_str_2 = r#"
    {
        "array": [
            { "v" : 1, "k" : 12 }
        ]
    }
    "#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(12), result.get_option("array:[0]:k"));
    assert_eq!(Some(33), result.get_option("array:[1]:k"));
}

#[test]
fn test_structs_of_arrays_build_json() {
    let config_str_1 = r#"
    {
        "structure" : {
            "a1" : [1, 42, 3],
            "a2" : [1, 2]
        }
    }
    "#
    .trim();

    let config_str_2 = r#"
    {
        "structure" : {
            "a1" : [11],
            "a2" : [4, 5, 3]
        }
    }
    "#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(11), result.get_option("structure:a1:[0]"));
    assert_eq!(Some(42), result.get_option("structure:a1:[1]"));
    assert_eq!(Some(3), result.get_option("structure:a1:[2]"));
    assert_eq!(None, result.get_option::<i32, &str>("structure:a1:[3]"));
    assert_eq!(Some(4), result.get_option("structure:a2:[0]"));
    assert_eq!(Some(5), result.get_option("structure:a2:[1]"));
    assert_eq!(Some(3), result.get_option("structure:a2:[2]"));
    assert_eq!(None, result.get_option::<i32, &str>("structure:a2:[3]"));
}

#[test]
fn test_triple_nested_map_build() {
    let config_str_1 = r#"
    {
        "key1" : {
            "key2" : {
                "key3" : true,
                "key4" : false
            }
        }
    }
    "#
    .trim();

    let config_str_2 = r#"
    {
        "key1" : {
            "key2" : {
                "key3" : false
            }
        }
    }
    "#
    .trim();

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(&config_str_1).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(false), result.get_option("key1:key2:key3"));
    assert_eq!(Some(false), result.get_option("key1:key2:key4"));
    assert_eq!(None, result.get_option::<i32, &str>("key1:key2:key5"));
}
