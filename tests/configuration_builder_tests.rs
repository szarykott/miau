use configuration_rs::builder::ConfigurationBuilder;
use configuration_rs::configuration::Configuration;
use configuration_rs::error::SourceDeserializationError;
use configuration_rs::source::InMemorySource;
use configuration_rs::compound_key;

#[test]
fn test() {
    let de_json = |st : String| {
        serde_json::from_str::<Configuration>(&st)
            .map_err(|e| SourceDeserializationError::SerdeError(e.to_string()))};

    let de_yaml = |st : String| {
        serde_yaml::from_str::<Configuration>(&st)
            .map_err(|e| SourceDeserializationError::SerdeError(e.to_string()))};        

    let mut builder = ConfigurationBuilder::new();
    builder.add(InMemorySource::from_str(&serde_json::json!({
        "number" : 1,
        "json1" : true,
        "map" : {
            "bool" : true
        }
    }).to_string()), de_json.clone());

    builder.add(InMemorySource::from_str(&serde_json::json!({
        "number" : 2,
        "json2" : true,
        "map" : {
            "nully" : "not null"
        }
    }).to_string()), de_json.clone());

    builder.add(InMemorySource::from_str(r#"
number: 3
yaml: true
map:
  nulla: ~"#.trim()), de_yaml.clone());

    let configuration = builder.build().unwrap();

    assert_eq!(Some(3), configuration.drill_get::<i8>(&compound_key!("number")));
    assert_eq!(Some(true), configuration.drill_get::<bool>(&compound_key!("yaml")));
    assert_eq!(Some(true), configuration.drill_get::<bool>(&compound_key!("json1")));
    assert_eq!(Some(true), configuration.drill_get::<bool>(&compound_key!("json2")));
    assert_eq!(Some(true), configuration.drill_get::<bool>(&compound_key!("map", "bool")));
    assert_eq!(Some("not null"), configuration.drill_get::<&str>(&compound_key!("map", "nully")));
    assert_eq!(None, configuration.drill_get::<&str>(&compound_key!("map", "nulla")))
}
