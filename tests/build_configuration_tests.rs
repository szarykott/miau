use configuration_rs::configuration::{Configuration, TypedValue};
use configuration_rs::key;
use std::collections::HashMap;

#[test]
fn build_tree_manually() {
    let mut root = HashMap::new();

    root.insert(
        "key1".to_string(),
        Configuration::Value(Some(TypedValue::String("value1".into()))),
    );
    root.insert(
        "key2".to_string(),
        Configuration::Array(vec![
            Configuration::Value(Some(TypedValue::String("value2".into()))),
            Configuration::Value(Some(TypedValue::String("value3".into()))),
            Configuration::Value(Some(TypedValue::Bool(true))),
            Configuration::Value(None),
        ]),
    );

    let _cfg = Configuration::Map(root);

    // println!("{}", serde_json::to_string_pretty(&_cfg).unwrap());

    // we got here we are all right!
    assert!(true)
}

#[test]
fn build_tree_from_yaml_1() {
    let tree: &str = r#"
key1: value1
key2:
- value2
- value3
- 1"#
        .trim();

    let configuration = serde_yaml::from_str::<Configuration>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(Some("value1".to_string()), cfg.drill_get(&key!["key1"]));
    assert_eq!(Some("value2".to_string()), cfg.drill_get(&key!["key2", 0u8]));
    assert_eq!(Some("value3".to_string()), cfg.drill_get(&key!["key2", 1u8]));
    assert_eq!(Some(1), cfg.drill_get(&key!["key2", 2u8]));
}

#[test]
fn build_tree_from_json_1() {
    let tree: &str = r#"
    {
        "menu": {
          "id": "file",
          "value": 1,
          "popup": {
            "menuitem": [
              {"value": 1.2, "onclick": null},
              {"value": true, "onclick": -12.1}
            ]
          }
        }
      }
    "#
    .trim();

    let configuration = serde_json::from_str::<Configuration>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(Some("file".to_string()), cfg.drill_get(&key!("menu", "id")));
    assert_eq!(Some(1), cfg.drill_get(&key!("menu", "value")));
    assert_eq!(
        Some(1.2f32),
        cfg.drill_get(&key!("menu", "popup", "menuitem", 0u8, "value"))
    );
    assert_eq!(
        None,
        cfg.drill_get::<i8>(&key!("menu", "popup", "menuitem", 0u8, "onclick"))
    );
    assert_eq!(
        Some(true),
        cfg.drill_get(&key!("menu", "popup", "menuitem", 1u8, "value"))
    );
    assert_eq!(
        Some(-12.1),
        cfg.drill_get(&key!("menu", "popup", "menuitem", 1u8, "onclick"))
    );
}
