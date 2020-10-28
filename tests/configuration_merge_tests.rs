use configuration_rs::{
    builder::ConfigurationBuilder, format::JsonDeserializer, source::InMemorySource,
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
    ),
    case(
        r#"{"array1" : []}"#,
        r#"{"array1" : [4,5,6]}"#,
        vec![4, 5, 6]
    ),
    case(
        r#"{"array1" : [4,5,6]}"#,
        r#"{"array1" : []}"#,
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
        .merge_owned()
        .unwrap()
        .try_into::<HashMap<String, Vec<i32>>>()
        .unwrap();

    assert_eq!(exp, result.remove("array1".into()).unwrap());
}
