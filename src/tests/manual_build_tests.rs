#![cfg(test)]

use crate::configuration::{Node, Value};
use std::collections::HashMap;

#[test]
fn build_tree_manually() {
    let mut root = HashMap::new();

    root.insert(
        "key1".to_string(),
        Node::Value(Some(Value::String("value1".into()))),
    );
    root.insert(
        "key2".to_string(),
        Node::Array(vec![
            Node::Value(Some(Value::String("value2".into()))),
            Node::Value(Some(Value::String("value3".into()))),
            Node::Value(Some(Value::Bool(true))),
            Node::Value(None),
        ]),
    );

    let _cfg = Node::Map(root);

    // we got here we are all right!
    assert!(true)
}
