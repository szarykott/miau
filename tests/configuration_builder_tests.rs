mod utils;

use configuration_rs::{
    builder::ConfigurationBuilder, format::JsonDeserializer, key, source::InMemorySource,
};
use rstest::rstest;
use std::collections::HashMap;

#[rstest(
    json1,
    json2,
    exp,
    case(
        r#"{"array1" : [1,2,3,4]}"#,
        r#"{"array1" : [4,5]}"#,
        vec![4, 5, 3, 4]
    ),
    case(
        r#"{"array1" : [1,2]}"#,
        r#"{"array1" : [4,5,6]}"#,
        vec![4, 5, 6]
    )
)]

fn test_arrays_are_merged_when_substituted(json1: &str, json2: &str, exp: Vec<i32>) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_str(json1.as_ref()),
        JsonDeserializer::new(),
    );
    builder.add(
        InMemorySource::from_str(json2.as_ref()),
        JsonDeserializer::new(),
    );

    let confiuration = builder.build().unwrap();

    let mut result = confiuration
        .merge()
        .unwrap()
        .try_into::<HashMap<String, Vec<i32>>>()
        .unwrap();

    assert_eq!(exp, result.remove("array1".into()).unwrap());
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : 2}"#, 2),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : 1}"#, 1),
    case(r#"{"value1" : false}"#, r#"{"value1" : 3}"#, 3),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : 4}"#, 4)
)]
fn test_type_to_integer_substitution(c1: &str, c2: &str, exp: isize) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_str(c1.as_ref()),
        JsonDeserializer::new(),
    );
    builder.add(
        InMemorySource::from_str(c2.as_ref()),
        JsonDeserializer::new(),
    );

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get::<isize>(&key!("value1"));
    assert_eq!(Some(exp), value);
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : 2.1}"#, 2.1f64),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : 1.1}"#, 1.1f64),
    case(r#"{"value1" : false}"#, r#"{"value1" : 3.1}"#, 3.1f64),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : 4.1}"#, 4.1f64)
)]
fn test_type_to_float_substitution(c1: &str, c2: &str, exp: f64) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_str(c1.as_ref()),
        JsonDeserializer::new(),
    );
    builder.add(
        InMemorySource::from_str(c2.as_ref()),
        JsonDeserializer::new(),
    );

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get::<f64>(&key!("value1"));
    assert_eq!(Some(exp), value);
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : false}"#, r#"{"value1" : true}"#, true),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : false}"#, false)
)]
fn test_type_to_bool_substitution(c1: &str, c2: &str, exp: bool) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_str(c1.as_ref()),
        JsonDeserializer::new(),
    );
    builder.add(
        InMemorySource::from_str(c2.as_ref()),
        JsonDeserializer::new(),
    );

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get::<bool>(&key!("value1"));
    assert_eq!(Some(exp), value);
}

#[rstest(
    c1,
    c2,
    exp,
    case(r#"{"value1" : 1}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : 1.2}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : false}"#, r#"{"value1" : "true"}"#, "true"),
    case(r#"{"value1" : "true"}"#, r#"{"value1" : "false"}"#, "false")
)]
fn test_type_to_string_substitution(c1: &str, c2: &str, exp: &str) {
    let mut builder = ConfigurationBuilder::default();

    builder.add(
        InMemorySource::from_str(c1.as_ref()),
        JsonDeserializer::new(),
    );
    builder.add(
        InMemorySource::from_str(c2.as_ref()),
        JsonDeserializer::new(),
    );

    let result = builder.build();

    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get::<String>(&key!("value1"));
    assert_eq!(Some(exp.to_string()), value);
}
