#![cfg(test)]

use configuration_rs::configuration::{ConfigurationNode, Value};
use std::collections::HashMap;

#[test]
fn build_tree_manually() {
    let mut root = HashMap::new();

    root.insert(
        "key1".to_string(),
        ConfigurationNode::Value(Some(Value::String("value1".into()))),
    );
    root.insert(
        "key2".to_string(),
        ConfigurationNode::Array(vec![
            ConfigurationNode::Value(Some(Value::String("value2".into()))),
            ConfigurationNode::Value(Some(Value::String("value3".into()))),
            ConfigurationNode::Value(Some(Value::Bool(true))),
            ConfigurationNode::Value(None),
        ]),
    );

    let _cfg = ConfigurationNode::Map(root);

    // we got here we are all right!
    assert!(true)
}
