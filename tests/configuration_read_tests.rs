mod utils;

use configuration_rs::{
    builder::ConfigurationBuilder, configuration::Configuration, error::ErrorCode, format::Json,
    key, source::InMemorySource,
};
use rstest::rstest;

#[test]
fn test_arrays_are_subsituted_when_config_is_built() {
    let json1 = r#"{"array1" : [1,2,3,4]}"#;
    let json2 = r#"{"array1" : [5,6]}"#;
    let json3 = r#"{"array1" : [7]}"#;

    let mut builder = ConfigurationBuilder::default();

    builder
        .add(InMemorySource::from_str(json1), Json::new())
        .add(InMemorySource::from_str(json2), Json::new())
        .add(InMemorySource::from_str(json3), Json::new());

    let confiuration = builder.build().unwrap();

    assert_eq!(Some(7), confiuration.get("array1:[0]"));
    assert_eq!(Some(6), confiuration.get("array1:[1]"));
    assert_eq!(Some(3), confiuration.get("array1:[2]"));
    assert_eq!(Some(4), confiuration.get("array1:[3]"));
}

#[test]
fn test_array_to_map_substitution() {
    let json1 = r#"{"key" : [7]}"#;
    let json2 = r#"{"key" : { "key" : 7 }}"#;

    let mut builder = ConfigurationBuilder::default();

    builder
        .add(InMemorySource::from_str(json1), Json::new())
        .add(InMemorySource::from_str(json2), Json::new());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(7), configuration.get("key:[0]"));
    assert_eq!(Some(7), configuration.get("key:key"));
}

#[test]
fn test_map_to_array_substitution() {
    let json1 = r#"{"key" : { "key" : 7 }}"#;
    let json2 = r#"{"key" : [7]}"#;

    let mut builder = ConfigurationBuilder::default();

    builder
        .add(InMemorySource::from_str(json1), Json::new())
        .add(InMemorySource::from_str(json2), Json::new());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(7), configuration.get("key:[0]"));
    assert_eq!(Some(7), configuration.get("key:key"));
}

#[test]
fn test_get_result_non_existing_key() {
    let json1 = r#"{"key" : { "key" : 7 }}"#;

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(json1), Json::new());

    let configuration = builder.build().unwrap();

    let value = configuration.get_result::<i32, &str>("value").unwrap();

    assert_eq!(None, value);
}

#[test]
fn test_get_result_wrong_key_type() {
    let json1 = r#"{"key" : { "key" : "not_a_number" }}"#;

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(json1), Json::new());

    let configuration = builder.build().unwrap();

    let value = configuration.get_result::<i32, &str>("key:key").unwrap();

    assert_eq!(None, value);
}

#[test]
fn test_get_result_key_unparsable() {
    let json1 = r#"{"key" : { "key" : "not_a_number" }}"#;

    let mut builder = ConfigurationBuilder::default();
    builder.add(InMemorySource::from_str(json1), Json::new());

    let configuration = builder.build().unwrap();

    let key = "key:[A]:key";
    let error = configuration.get_result::<i32, &str>(key).unwrap_err();

    let error_string = error.to_string();

    assert!(std::matches!(error.get_code(), ErrorCode::ParsingError(..)));
    assert!(error_string.contains(key));
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

    builder.add(InMemorySource::from_str(c1), Json::new());
    builder.add(InMemorySource::from_str(c2), Json::new());

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(Some(exp), result.get("value1"));
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

    builder.add(InMemorySource::from_str(c1), Json::new());
    builder.add(InMemorySource::from_str(c2), Json::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get("value1"));
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

    builder.add(InMemorySource::from_str(c1), Json::new());
    builder.add(InMemorySource::from_str(c2), Json::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get("value1"));
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

    builder.add(InMemorySource::from_str(c1), Json::new());
    builder.add(InMemorySource::from_str(c2), Json::new());

    let result = builder.build().unwrap();

    assert_eq!(Some(exp), result.get("value1"));
}

#[test]
fn test_single_value_integer_config_build_json() {
    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add_provider(serde_json::from_str::<Configuration>("1").unwrap())
        .add_provider(serde_json::from_str::<Configuration>("2").unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2i32), result.get(""));
    assert_eq!(Some(2i32), result.get(key!()));
}

#[test]
fn test_single_map_entry_config_build_json() {
    let mut builder = ConfigurationBuilder::default();

    let result = builder
        .add_provider(serde_json::from_str::<Configuration>(r#"{"value" : 1}"#).unwrap())
        .add_provider(serde_json::from_str::<Configuration>(r#"{"value" : 2}"#).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2), result.get("value"));
    assert_eq!(Some(2), result.get(key!("value")));
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

    assert_eq!(Some("2"), result.get("value"));
    assert_eq!(Some(2), result.get("value"));
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

    assert_eq!(Some(1), result.get("value1"));
    assert_eq!(Some(2), result.get("value2"));
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

    assert_eq!(Some(2), result.get("[0]"));
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

    assert_eq!(Some("Andrew"), result.get("firstName"));
    assert_eq!(Some("Smith"), result.get("lastName"));
    assert_eq!(Some(false), result.get("isAlive"));
    assert_eq!(Some("Knowhere"), result.get("address:streetAddress"));
    assert_eq!(Some("work"), result.get("phoneNumbers:[0]:type"));
    assert_eq!(Some(true), result.get("spouse"));
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

    assert_eq!(Some(12), result.get("array:[0]:k"));
    assert_eq!(Some(33), result.get("array:[1]:k"));
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

    assert_eq!(Some(11), result.get("structure:a1:[0]"));
    assert_eq!(Some(42), result.get("structure:a1:[1]"));
    assert_eq!(Some(3), result.get("structure:a1:[2]"));
    assert_eq!(None, result.get::<i32, &str>("structure:a1:[3]"));
    assert_eq!(Some(4), result.get("structure:a2:[0]"));
    assert_eq!(Some(5), result.get("structure:a2:[1]"));
    assert_eq!(Some(3), result.get("structure:a2:[2]"));
    assert_eq!(None, result.get::<i32, &str>("structure:a2:[3]"));
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

    assert_eq!(Some(false), result.get("key1:key2:key3"));
    assert_eq!(Some(false), result.get("key1:key2:key4"));
    assert_eq!(None, result.get::<i32, &str>("key1:key2:key5"));
}
