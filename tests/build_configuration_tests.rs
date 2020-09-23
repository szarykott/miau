use configuration_rs::configuration::{Configuration, ConfigurationNode, TypedValue};
use configuration_rs::{error::ConfigurationAccessError, get_typed_value, get_value};
use std::collections::HashMap;

#[test]
fn build_tree_manually() {
    let mut root = HashMap::new();

    root.insert(
        "key1".to_string(),
        ConfigurationNode::Value(Some(TypedValue::String("value1".into()))),
    );
    root.insert(
        "key2".to_string(),
        ConfigurationNode::Array(vec![
            ConfigurationNode::Value(Some(TypedValue::String("value2".into()))),
            ConfigurationNode::Value(Some(TypedValue::String("value3".into()))),
            ConfigurationNode::Value(Some(TypedValue::Bool(true))),
            ConfigurationNode::Value(None),
        ]),
    );

    let _cfg = Configuration {
        root: ConfigurationNode::Map(root),
    };

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

    let configuration = configuration.unwrap();

    let v1: &str = get_value!(&configuration, "key1").unwrap().unwrap();
    let v2: &str = get_value!(&configuration, "key2", "0").unwrap().unwrap();
    let v3: &str = get_value!(&configuration, "key2", "1").unwrap().unwrap();
    let v4: i64 = get_value!(&configuration, "key2", "2").unwrap().unwrap();

    assert_eq!("value1", v1);
    assert_eq!("value2", v2);
    assert_eq!("value3", v3);
    assert_eq!(1, v4);
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

    let root = configuration.unwrap();

    assert_eq!(
        "file",
        get_typed_value!(&root, &str => "menu", "id")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        1,
        get_typed_value!(&root, i8 => "menu", "value")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        1.2f32,
        get_typed_value!(&root, f32 => "menu", "popup", "menuitem", "0", "value")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        None,
        get_typed_value!(&root, &str => "menu", "popup", "menuitem", "0", "onclick").unwrap()
    );
    assert_eq!(
        true,
        get_typed_value!(&root, bool =>  "menu", "popup", "menuitem", "1", "value")
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        -12.1,
        get_typed_value!(&root, f64 => "menu", "popup", "menuitem", "1", "onclick")
            .unwrap()
            .unwrap()
    );
}
