use configuration_rs::builder::ConfigurationBuilder;
use configuration_rs::key;
use configuration_rs::source::InMemorySource;
use configuration_rs::format::{
    YamlDeserializer, JsonDeserializer
};

#[test]
fn test() {
    let mut builder = ConfigurationBuilder::new();
    builder.add(
        InMemorySource::from_str(
            &serde_json::json!({
                "number" : 1,
                "json1" : true,
                "map" : {
                    "bool" : true
                }
            }).to_string(),
        ),
        JsonDeserializer::new(),
    );

    builder.add(
        InMemorySource::from_str(
            &serde_json::json!({
                "number" : 2,
                "json2" : true,
                "map" : {
                    "nully" : "not null"
                }
            })
            .to_string(),
        ),
        JsonDeserializer::new(),
    );

    builder.add(
        InMemorySource::from_str(
            r#"
number: 3
yaml: true
map:
  nulla: ~"#
                .trim(),
        ),
        YamlDeserializer::new(),
    );

    let cfg = builder.build().unwrap();

    assert_eq!(Some(3i8), cfg.drill_get(&key!("number")));
    assert_eq!(Some(true), cfg.drill_get(&key!("yaml")));
    assert_eq!(Some(true), cfg.drill_get(&key!("json1")));
    assert_eq!(Some(true), cfg.drill_get(&key!("json2")));
    assert_eq!(Some(true), cfg.drill_get(&key!("map", "bool")));
    assert_eq!(Some("not null"), cfg.drill_get(&key!("map", "nully")));
    assert_eq!(None, cfg.drill_get::<&str>(&key!("map", "nulla")))
}