use configuration_rs::{
    key,
    builder::ConfigurationBuilder,
    source::InMemorySource,
    format::{
        YamlDeserializer, JsonDeserializer
    },
    configuration::Configuration,
    error::ConfigurationMergeError
};
use rstest::{
    rstest, fixture
};

#[rstest(c1, c2, c3, exp,
    case(r#"{"value1" : 1}"#, r#"{"value2" : 2}"#, r#"{"value3" : 3}"#, r#"{"value1":1,"value2":2,"value3":3}"#),
    case(r#"{"value1" : "1"}"#, r#"{"value2" : "2"}"#, r#"{"value3" : "3"}"#, r#"{"value1":"1","value2":"2","value3":"3"}"#),
    case(r#"{"value1" : "1"}"#, r#"{"value1" : "2"}"#, r#"{"value3" : 3}"#, r#"{"value1":"2","value3":3}"#),
    case(
        r#"{
            "map1" : {
                "v1" : 1,
                "v2" : 5
            }
        }"#, 
        r#"{
            "map2" : {},
            "map1" : {
                "v1" : 2
            }
        }"#, 
        r#"{
            "map1" : {
                "v1" : 3,
                "v3" : 4
            }
        }"#, 
        r#"{
            "map2" : {},
            "map1" : {
                "v1" : 3,
                "v2" : 5,
                "v3" : 4
            }
        }"#
    ),
    case(
        r#"{
            "map1" : {
                "v1" : 1,
                "v2" : [true, true, false]
            }
        }"#, 
        r#"{
            "map2" : {},
            "map1" : {
                "v1" : 2,
                "v2" : [false, true, false]
            }
        }"#, 
        r#"{
            "map1" : {
                "v1" : 3,
                "v3" : 4
            }
        }"#, 
        r#"{
            "map2" : {},
            "map1" : {
                "v1" : 3,
                "v2" : [false, true, false],
                "v3" : 4
            }
        }"#
    ),
    case(
        r#"{
            "map1" : {
                "v1" : 1,
                "v2" : [true, true, false]
            }
        }"#, 
        r#"{
            "map2" : {
                "mapi1" : {
                    "mapi2" : {
                        "val" : null
                    },
                    "mapi3" : {
                        "val" : null
                    }
                }
            },
            "map1" : {
                "v1" : 2,
                "v2" : [false, true, false]
            }
        }"#, 
        r#"{
            "map2" : {
                "mapi1" : {
                    "mapi2" : {
                        "val" : 1
                    }
                }
            },
            "map1" : {
                "v1" : 3,
                "v3" : 4
            }
        }"#, 
        r#"{
            "map2" : {
                "mapi1" : {
                    "mapi2" : {
                        "val" : 1
                    },
                    "mapi3" : {
                        "val" : null
                    }
                }
            },
            "map1" : {
                "v1" : 3,
                "v2" : [false, true, false],
                "v3" : 4
            }
        }"#
    )
)]
fn test_basic_json_data_success(c1 : &str, c2 : &str, c3 : &str, exp : &str) {
    let mut builder = ConfigurationBuilder::new();
    
    builder.add(InMemorySource::from_str(c1.as_ref()), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2.as_ref()), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c3.as_ref()), JsonDeserializer::new());   
    
    let result = builder.build().unwrap();
    let expected = serde_json::from_str::<Configuration>(exp.as_ref()).unwrap();   
    
    assert_eq!(expected, result);
}

#[rstest(c1, c2,
    case(r#"{"value1" : 1}"#, r#"{"value2" : 2}"#),
    case(r#"{"value1" : 1}"#, r#"{"value2" : 2}"#)
)]
fn test_invalid_type_substitution(c1 : &str, c2 : &str) {
    let mut builder = ConfigurationBuilder::new();

    builder.add(InMemorySource::from_str(c1.as_ref()), JsonDeserializer::new());
    builder.add(InMemorySource::from_str(c2.as_ref()), JsonDeserializer::new());

}

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