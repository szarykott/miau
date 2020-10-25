#![cfg(test)]

use crate::configuration::Node;
use crate::key;

#[test]
fn build_tree_from_yaml_1() {
    let tree: &str = r#"
key1: value1
key2:
- value2
- value3
- 1"#
        .trim();

    let configuration = serde_yaml::from_str::<Node>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(Some("value1".to_string()), cfg.get_option(&key!["key1"]));
    assert_eq!(
        Some("value2".to_string()),
        cfg.get_option(&key!["key2", 0u8])
    );
    assert_eq!(
        Some("value3".to_string()),
        cfg.get_option(&key!["key2", 1u8])
    );
    assert_eq!(Some(1), cfg.get_option(&key!["key2", 2u8]));
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

    let configuration = serde_json::from_str::<Node>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(
        Some("file".to_string()),
        cfg.get_option(&key!("menu", "id"))
    );
    assert_eq!(Some(1), cfg.get_option(&key!("menu", "value")));
    assert_eq!(
        Some(1.2f32),
        cfg.get_option(&key!("menu", "popup", "menuitem", 0u8, "value"))
    );
    assert_eq!(
        None,
        cfg.get_option::<i8>(&key!("menu", "popup", "menuitem", 0u8, "onclick"))
    );
    assert_eq!(
        Some(true),
        cfg.get_option(&key!("menu", "popup", "menuitem", 1u8, "value"))
    );
    assert_eq!(
        Some(-12.1),
        cfg.get_option(&key!("menu", "popup", "menuitem", 1u8, "onclick"))
    );
}
