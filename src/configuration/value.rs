use crate::error::{ConfigurationError, ErrorCode};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Bool(bool),
    SignedInteger(i64),
    Float(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(v) => write!(f, "String : {}", v),
            Value::Bool(v) => write!(f, "Bool : {}", v),
            Value::SignedInteger(v) => write!(f, "SignedInteger : {}", v),
            Value::Float(v) => write!(f, "Float : {}", v),
        }
    }
}

macro_rules! try_from_for_int {
    ($($t:ty),*) => {
        $(
            impl TryFrom<&Value> for $t {
                type Error = ConfigurationError;

                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::String(v) => v.parse::<$t>()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "string".into()).into()),
                        Value::Bool(v) => Ok(if v == &true { 1 as $t } else { 0 as $t }),
                        Value::SignedInteger(v) => (*v).try_into()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "i64".into()).into()),
                        Value::Float(v) => {
                            if *v <= <$t>::MAX as f64 {
                                Ok(*v as $t)
                            } else {
                                Err(ErrorCode::UnexpectedValueType(stringify!($t).into(), "f64".into()).into())
                            }
                        }
                    }
                }
            }
        )*
    };
}
try_from_for_int!(i8, i16, i32, i64, isize);

macro_rules! try_from_for_float {
    ($($t:ty),*) => {
        $(
            impl TryFrom<&Value> for $t {
                type Error = ConfigurationError;

                fn try_from(value: &Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::String(v) => v.parse::<$t>()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "string".into()).into()),
                        Value::Bool(v) => Ok(if v == &true { 1 as $t } else { 0 as $t }),
                        Value::SignedInteger(v) => Ok(*v as $t),
                        Value::Float(v) => Ok(*v as $t)
                    }
                }
            }
        )*
    };
}
try_from_for_float!(f32, f64);

impl TryFrom<&Value> for bool {
    type Error = ConfigurationError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => {
                let vlc = v.to_lowercase();
                if vlc == "1" || vlc == "true" {
                    Ok(true)
                } else if vlc == "0" || vlc == "false" {
                    Ok(false)
                } else {
                    Err(ErrorCode::UnexpectedValueType("bool".into(), "string".into()).into())
                }
            }
            Value::Bool(v) => Ok(*v),
            Value::SignedInteger(v) => {
                if v == &1 {
                    Ok(true)
                } else if v == &0 {
                    Ok(false)
                } else {
                    Err(ErrorCode::UnexpectedValueType("bool".into(), "i64".into()).into())
                }
            }
            Value::Float(v) => {
                if v == &1f64 {
                    Ok(true)
                } else if v == &0f64 {
                    Ok(false)
                } else {
                    Err(ErrorCode::UnexpectedValueType("bool".into(), "f64".into()).into())
                }
            }
        }
    }
}

// TODO: This could be From
impl TryFrom<&Value> for String {
    type Error = ConfigurationError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(match value {
            Value::String(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::SignedInteger(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
        })
    }
}

impl<'conf> TryFrom<&'conf Value> for &'conf str {
    type Error = ConfigurationError;

    fn try_from(value: &'conf Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.as_str()),
            _ => Err(ErrorCode::UnexpectedValueType("".into(), "".into()).into()), //TODO: Fix it
        }
    }
}
