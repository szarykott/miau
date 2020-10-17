use crate::error::{ConfigurationError, ErrorCode};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TypedValue {
    String(String),
    Bool(bool),
    Float(f64),
    SignedInteger(i64),
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedValue::String(v) => write!(f, "String : {}", v),
            TypedValue::Bool(v) => write!(f, "Bool : {}", v),
            TypedValue::SignedInteger(v) => write!(f, "SignedInteger : {}", v),
            TypedValue::Float(v) => write!(f, "Float : {}", v),
        }
    }
}

macro_rules! try_from_for_int {
    ($($t:ty),*) => {
        $(
            impl TryFrom<&TypedValue> for $t {
                type Error = ConfigurationError;

                fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                    match value {
                        TypedValue::String(v) => v.parse::<$t>()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "string".into()).into()),
                        TypedValue::Bool(v) => Ok(if v == &true { 1 as $t } else { 0 as $t }),
                        TypedValue::SignedInteger(v) => (*v).try_into()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "i64".into()).into()),
                        TypedValue::Float(v) => {
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
            impl TryFrom<&TypedValue> for $t {
                type Error = ConfigurationError;

                fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                    match value {
                        TypedValue::String(v) => v.parse::<$t>()
                            .map_err(|_| ErrorCode::UnexpectedValueType(stringify!($t).into(), "string".into()).into()),
                        TypedValue::Bool(v) => Ok(if v == &true { 1 as $t } else { 0 as $t }),
                        TypedValue::SignedInteger(v) => Ok(*v as $t),
                        TypedValue::Float(v) => Ok(*v as $t)
                    }
                }
            }
        )*
    };
}
try_from_for_float!(f32, f64);

impl TryFrom<&TypedValue> for bool {
    type Error = ConfigurationError;

    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
        match value {
            TypedValue::String(v) => {
                let vlc = v.to_lowercase();
                if vlc == "1" || vlc == "true" {
                    Ok(true)
                } else if vlc == "0" || vlc == "false" {
                    Ok(false)
                } else {
                    Err(ErrorCode::UnexpectedValueType("bool".into(), "string".into()).into())
                }
            }
            TypedValue::Bool(v) => Ok(*v),
            TypedValue::SignedInteger(v) => {
                if v == &1 {
                    Ok(true)
                } else if v == &0 {
                    Ok(false)
                } else {
                    Err(ErrorCode::UnexpectedValueType("bool".into(), "i64".into()).into())
                }
            }
            TypedValue::Float(v) => {
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

impl TryFrom<&TypedValue> for String {
    type Error = ConfigurationError;

    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
        Ok(match value {
            TypedValue::String(v) => v.to_string(),
            TypedValue::Bool(v) => v.to_string(),
            TypedValue::SignedInteger(v) => v.to_string(),
            TypedValue::Float(v) => v.to_string(),
        })
    }
}
