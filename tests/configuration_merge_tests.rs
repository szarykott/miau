use configuration_rs::compound_key;
use configuration_rs::configuration::Configuration;
use configuration_rs::error::ConfigurationMergeError;

#[test]
fn test_single_value_integer_config_merge_json() {
    let config_str_1 = "1";
    let config_str_2 = "2";

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(2, result.get_value::<isize>().unwrap().unwrap());
}

#[test]
fn test_single_map_entry_config_merge_json() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : 2}"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(Some(2), result.drill_get::<i8>(&compound_key!("value")));
}

#[test]
fn test_single_map_entry_config_merge_json_wrong_type() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : "2"}"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2);

    assert_eq!(ConfigurationMergeError::IncompatibleValueSubstitution, result.unwrap_err());
}

#[test]
fn test_single_array_entry_config_merge_json() {
    let config_str_1 = r#"[1]"#;
    let config_str_2 = r#"[2]"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(Some(2), result.drill_get(&compound_key!(0u8)));
}

#[test]
fn test_complex_map_config_merge_json() {
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

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(
        Some("Andrew"),
        result.drill_get(&compound_key!("firstName"))
    );
    assert_eq!(Some("Smith"), result.drill_get(&compound_key!("lastName")));
    assert_eq!(Some(false), result.drill_get(&compound_key!("isAlive")));
    assert_eq!(
        Some("Knowhere"),
        result.drill_get(&compound_key!("address", "streetAddress"))
    );
    assert_eq!(
        Some("work"),
        result.drill_get(&compound_key!("phoneNumbers", 0u32, "type"))
    );
    assert_eq!(Some(true), result.drill_get(&compound_key!("spouse")));
}

#[test]
fn test_array_config_merge_json() {
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

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(
        Some(12),
        result.drill_get(&compound_key!("array", 0u8, "k"))
    );
    assert_eq!(
        None,
        result.drill_get::<i32>(&compound_key!("array", 1u8, "k"))
    );
}
