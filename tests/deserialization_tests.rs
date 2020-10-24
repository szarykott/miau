use configuration_rs::configuration::Node;
use serde::Deserialize;

#[test]
fn test_deserialization_all_simple_types() {
    #[derive(Deserialize)]
    struct Config {
        integer64: i64,
        integer32: i32,
        integer16: i16,
        integer8: i8,
        uinteger64: u64,
        uinteger32: u32,
        uinteger16: u16,
        uinteger8: u8,
        boolean: bool,
        string_owned: String,
        float32: f32,
        float64: f64,
        unit: (),
    }

    let config_str = serde_json::json!({
        "integer64": 63,
        "integer32": 31,
        "integer16": 15,
        "integer8": 7,
        "uinteger64": 63,
        "uinteger32": 31,
        "uinteger16": 15,
        "uinteger8": 7,
        "boolean" : true,
        "string_owned" : "owned",
        "str_ref" : "strref",
        "float32" : 1.1,
        "float64" : 1.2,
        "unit" : null
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(63, config.integer64);
    assert_eq!(31, config.integer32);
    assert_eq!(15, config.integer16);
    assert_eq!(7, config.integer8);
    assert_eq!(63, config.uinteger64);
    assert_eq!(31, config.uinteger32);
    assert_eq!(15, config.uinteger16);
    assert_eq!(7, config.uinteger8);
    assert_eq!(true, config.boolean);
    assert_eq!("owned".to_string(), config.string_owned);
    assert_eq!(1.1, config.float32);
    assert_eq!(1.2, config.float64);
    assert_eq!((), config.unit);
}

#[test]
fn test_deserialization_struct_with_map() {
    #[derive(Deserialize)]
    struct Config {
        inner: ConfigInner,
    }

    #[derive(Deserialize)]
    struct ConfigInner {
        value: i32,
    }

    let config_str = serde_json::json!({
        "inner": {
            "value" : 42
        },
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(42, config.inner.value);
}

#[test]
fn test_deserialization_struct_with_array() {
    #[derive(Deserialize)]
    struct Config {
        inner: Vec<i32>,
    }

    let config_str = serde_json::json!({
        "inner": [1, 2, 3]
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert!(vec![1, 2, 3].iter().eq(config.inner.iter()));
}

#[test]
fn test_deserialization_struct_with_array_of_structs() {
    #[derive(Deserialize)]
    struct Config {
        inner: Vec<ConfigInner>,
    }

    #[derive(Deserialize, PartialEq)]
    struct ConfigInner {
        value: i32,
    }

    let config_str = serde_json::json!({
        "inner": [
            {"value" : 1},
            {"value" : 2},
            {"value" : 3},
        ]
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert!(vec![
        ConfigInner { value: 1 },
        ConfigInner { value: 2 },
        ConfigInner { value: 3 }
    ]
    .iter()
    .eq(config.inner.iter()));
}

#[test]
fn test_deserialization_struct_with_array_of_structs_transparent() {
    #[derive(Deserialize)]
    struct Config {
        inner: Vec<ConfigInner>,
    }

    #[derive(Deserialize, PartialEq)]
    #[serde(transparent)]
    struct ConfigInner {
        value: i32,
    }

    let config_str = serde_json::json!({
        "inner": [
            1, 2, 3
        ]
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert!(vec![
        ConfigInner { value: 1 },
        ConfigInner { value: 2 },
        ConfigInner { value: 3 }
    ]
    .iter()
    .eq(config.inner.iter()));
}

#[derive(Deserialize, PartialEq, Debug)]
enum DaEnum {
    Unit,
    Newtype(i32),
    Tuple(i32, i32),
    Structo { value: i32 },
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum DaEnumUntagged {
    Unit,
    Newtype(f32),
    Tuple(f32, i32),
    Structo { value: i32 },
}

#[test]
fn test_deserialization_enum_unit_variant() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnum,
    }

    let config_str = serde_json::json!({
        "enumeration": "Unit",
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnum::Unit, config.enumeration);
}

#[test]
fn test_deserialization_enum_unit_variant_untagged() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnumUntagged,
    }

    let config_str = serde_json::json!({
        "enumeration": null,
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnumUntagged::Unit, config.enumeration);
}

#[test]
fn test_deserialization_enum_newtype_variant() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnum,
    }

    let config_str = serde_json::json!({
        "enumeration": {
            "Newtype" : 42
        },
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnum::Newtype(42i32), config.enumeration);
}

#[test]
fn test_deserialization_enum_newtype_variant_untagged() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnumUntagged,
    }

    let config_str = serde_json::json!({
        "enumeration": 42,
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnumUntagged::Newtype(42f32), config.enumeration);
}

#[test]
fn test_deserialization_enum_tuple_variant() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnum,
    }

    let config_str = serde_json::json!({
        "enumeration": {
            "Tuple" : [1, 2]
        },
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnum::Tuple(1, 2), config.enumeration);
}

#[test]
fn test_deserialization_enum_tuple_variant_untagged() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnumUntagged,
    }

    let config_str = serde_json::json!({
        "enumeration": [1, 2],
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnumUntagged::Tuple(1f32, 2), config.enumeration);
}

#[test]
fn test_deserialization_enum_struct_variant() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnum,
    }

    let config_str = serde_json::json!({
        "enumeration": {
            "Structo" : {
                "value" : 3
            }
        },
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnum::Structo { value: 3 }, config.enumeration);
}

#[test]
fn test_deserialization_enum_struct_variant_untagged() {
    #[derive(Deserialize)]
    struct Config {
        enumeration: DaEnumUntagged,
    }

    let config_str = serde_json::json!({
        "enumeration": {
            "value" : 3
        },
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(DaEnumUntagged::Structo { value: 3 }, config.enumeration);
}

#[test]
fn test_deserialization_option() {
    #[derive(Deserialize)]
    struct Config {
        some: Option<f64>,
        none: Option<i16>,  // value will be null
        none2: Option<i16>, // value will be missing
    }

    let config_str = serde_json::json!({
        "some": 3,
        "none": null
    })
    .to_string();

    let root = serde_json::from_str::<Node>(&config_str).unwrap();

    let config = root.try_into::<Config>().unwrap();

    assert_eq!(Some(3f64), config.some);
    assert_eq!(None, config.none);
    assert_eq!(None, config.none2);
}
