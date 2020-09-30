use configuration_rs::configuration::Configuration;
use configuration_rs::key;

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

    assert_eq!(Some(2), result.drill_get(&key!("value")));
}

#[test]
fn test_single_map_entry_config_merge_json_wrong_type() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : "2"}"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2);

    // assert_eq!(
    //     ConfigurationMergeError::IncompatibleValueSubstitution,
    //     result.unwrap_err()
    // );
}

#[test]
fn test_two_map_entries_config_merge_json() {
    let config_str_1 = r#"{"value1" : 1}"#;
    let config_str_2 = r#"{"value2" : 2}"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(Some(1), result.drill_get::<i8>(&key!("value1")));
    assert_eq!(Some(2), result.drill_get::<i8>(&key!("value2")));
}

#[test]
fn test_single_array_entry_config_merge_json() {
    let config_str_1 = r#"[1]"#;
    let config_str_2 = r#"[2]"#;

    let configuration1 = serde_json::from_str::<Configuration>(&config_str_1).unwrap();
    let configuration2 = serde_json::from_str::<Configuration>(&config_str_2).unwrap();

    let result = Configuration::merge(configuration1, configuration2).unwrap();

    assert_eq!(Some(2), result.drill_get(&key!(0u8)));
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

    assert_eq!(Some("Andrew".to_string()), result.drill_get(&key!("firstName")));
    assert_eq!(Some("Smith".to_string()), result.drill_get(&key!("lastName")));
    assert_eq!(Some(false), result.drill_get(&key!("isAlive")));
    assert_eq!(
        Some("Knowhere".to_string()),
        result.drill_get(&key!("address", "streetAddress"))
    );
    assert_eq!(
        Some("work".to_string()),
        result.drill_get(&key!("phoneNumbers", 0u32, "type"))
    );
    assert_eq!(Some(true), result.drill_get(&key!("spouse")));
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

    assert_eq!(Some(12), result.drill_get(&key!("array", 0u8, "k")));
    assert_eq!(None, result.drill_get::<i32>(&key!("array", 1u8, "k")));
}
