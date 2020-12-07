use miau::{
    configuration::Value,
    error::{ConfigurationError, ErrorCode},
};
use std::convert::TryInto;

#[test]
fn test_float_variant_conversion() {
    let value = Value::Float(1.23);

    let flt: f64 = (&value).try_into().unwrap();
    assert_eq!(1.23, flt);

    let string_owned: String = (&value).try_into().unwrap();
    assert_eq!("1.23".to_string(), string_owned);

    // due to memory model used it is not possible
    let string_ref: Result<&str, ConfigurationError> = (&value).try_into();
    let error = string_ref.unwrap_err();
    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));

    let int: i32 = (&value).try_into().unwrap();
    assert_eq!(1, int);

    let boolean: Result<bool, ConfigurationError> = (&value).try_into();
    let error = boolean.unwrap_err();
    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));

    // a little tweak
    let value = Value::Float(1f64);
    let boolean: bool = (&value).try_into().unwrap();
    assert_eq!(true, boolean);
}

#[test]
fn test_string_variant_conversion() {
    let strv = Value::String("word".into());

    let string_owned: String = (&strv).try_into().unwrap();
    assert_eq!("word".to_string(), string_owned);

    let string_ref: &str = (&strv).try_into().unwrap();
    assert_eq!("word", string_ref);

    let boolean: Result<bool, ConfigurationError> = (&strv).try_into();
    let error = boolean.unwrap_err();
    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));

    let int: Result<i32, ConfigurationError> = (&strv).try_into();
    let error = int.unwrap_err();
    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));

    // little tweak
    let strv = Value::String("1".into());

    let int: i32 = (&strv).try_into().unwrap();
    assert_eq!(1, int);

    let flt: f32 = (&strv).try_into().unwrap();
    assert_eq!(1f32, flt);

    // little tweak
    let strv = Value::String("true".into());
    let boolean: bool = (&strv).try_into().unwrap();
    assert_eq!(true, boolean);
}

#[test]
fn test_integer_variant_conversion() {
    let value = Value::SignedInteger(1);

    let int: i32 = (&value).try_into().unwrap();
    assert_eq!(1, int);

    let flt: f64 = (&value).try_into().unwrap();
    assert_eq!(1f64, flt);

    let string_owned: String = (&value).try_into().unwrap();
    assert_eq!("1".to_string(), string_owned);

    // due to memory model used it is not possible
    let string_ref: Result<&str, ConfigurationError> = (&value).try_into();
    let error = string_ref.unwrap_err();
    assert!(std::matches!(
        error.get_code(),
        ErrorCode::WrongValueType(..)
    ));

    let boolean: bool = (&value).try_into().unwrap();
    assert_eq!(true, boolean);
}

#[test]
fn test_boolean_variant_conversion() {
    let value = Value::Bool(true);

    let int: i32 = (&value).try_into().unwrap();
    assert_eq!(1, int);

    let flt: f64 = (&value).try_into().unwrap();
    assert_eq!(1f64, flt);

    let string_owned: String = (&value).try_into().unwrap();
    assert_eq!("true".to_string(), string_owned);

    // it is actually possible with bools
    let string_ref: &str = (&value).try_into().unwrap();
    assert_eq!("true", string_ref);

    let boolean: bool = (&value).try_into().unwrap();
    assert_eq!(true, boolean);
}
