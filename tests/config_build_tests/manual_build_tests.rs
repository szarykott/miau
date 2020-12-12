use miau::configuration::{ConfigurationTree, Value};
use std::collections::HashMap;

#[test]
fn build_tree_manually() {
    let mut root = HashMap::new();

    root.insert(
        "key1".to_string(),
        ConfigurationTree::Value(Some(Value::String("value1".into()))),
    );
    root.insert(
        "key2".to_string(),
        ConfigurationTree::Array(vec![
            ConfigurationTree::Value(Some(Value::String("value2".into()))),
            ConfigurationTree::Value(Some(Value::String("value3".into()))),
            ConfigurationTree::Value(Some(Value::Bool(true))),
            ConfigurationTree::Value(None),
        ]),
    );

    let _cfg = ConfigurationTree::Map(root);

    // we got here we are all right!
    assert!(true)
}
