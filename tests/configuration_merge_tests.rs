use configuration_rs::key;
use configuration_rs::{builder::ConfigurationBuilder, configuration::Node};

#[test]
fn test_single_value_integer_config_merge_json() {
    let config_str_1 = "1";
    let config_str_2 = "2";

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(2, result.get_option::<isize>(&key!()).unwrap());
}

#[test]
fn test_single_map_entry_config_merge_json() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : 2}"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2), result.get_option(&key!("value")));
}

#[test]
fn test_single_map_entry_config_merge_json_wrong_type() {
    let config_str_1 = r#"{"value" : 1}"#;
    let config_str_2 = r#"{"value" : "2"}"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some("2"), result.get_option(&key!("value")))
}

#[test]
fn test_two_map_entries_config_merge_json() {
    let config_str_1 = r#"{"value1" : 1}"#;
    let config_str_2 = r#"{"value2" : 2}"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(1), result.get_option(&key!("value1")));
    assert_eq!(Some(2), result.get_option(&key!("value2")));
}

#[test]
fn test_single_array_entry_config_merge_json() {
    let config_str_1 = r#"[1]"#;
    let config_str_2 = r#"[2]"#;

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(2), result.get_option(&key!(0u8)));
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

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some("Andrew"), result.get_option(&key!("firstName")));
    assert_eq!(Some("Smith"), result.get_option(&key!("lastName")));
    assert_eq!(Some(false), result.get_option(&key!("isAlive")));
    assert_eq!(
        Some("Knowhere"),
        result.get_option(&key!("address", "streetAddress"))
    );
    assert_eq!(
        Some("work"),
        result.get_option(&key!("phoneNumbers", 0u32, "type"))
    );
    assert_eq!(Some(true), result.get_option(&key!("spouse")));
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

    let mut builder = ConfigurationBuilder::default();
    let result = builder
        .add_existing(serde_json::from_str::<Node>(&config_str_1).unwrap())
        .add_existing(serde_json::from_str::<Node>(&config_str_2).unwrap())
        .build()
        .unwrap();

    assert_eq!(Some(12), result.get_option(&key!("array", 0u8, "k")));
    assert_eq!(Some(33), result.get_option::<i32>(&key!("array", 1u8, "k")));
}
