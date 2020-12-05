#![cfg(test)]

use configuration_rs::configuration::ConfigurationNode;

#[test]
fn build_tree_from_yaml_1() {
    let tree: &str = r#"
key1: value1
key2:
- value2
- value3
- 1"#
        .trim();

    let configuration = serde_yaml::from_str::<ConfigurationNode>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(Some("value1".to_string()), cfg.get("key1"));
    assert_eq!(Some("value2".to_string()), cfg.get("key2:[0]"));
    assert_eq!(Some("value3".to_string()), cfg.get("key2:[1]"));
    assert_eq!(Some(1), cfg.get("key2:[2]"));
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

    let configuration = serde_json::from_str::<ConfigurationNode>(&tree);

    assert!(configuration.is_ok());

    let cfg = configuration.unwrap();

    assert_eq!(Some("file".to_string()), cfg.get("menu:id"));
    assert_eq!(Some(1), cfg.get("menu:value"));
    assert_eq!(Some(1.2f32), cfg.get("menu:popup:menuitem:[0]:value"));
    assert_eq!(None, cfg.get::<i8, &str>("menu:popup:menuitem:[0]:onclick"));
    assert_eq!(Some(true), cfg.get("menu:popup:menuitem:[1]:value"));
    assert_eq!(Some(-12.1), cfg.get("menu:popup:menuitem:[1]:onclick"));
}
