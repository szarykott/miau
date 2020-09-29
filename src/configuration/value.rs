use crate::error::{ConfigurationError, ErrorCode};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::mem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TypedValue {
    String(String),
    Bool(bool),
    SignedInteger(i64),
    Float(f64),
}

impl TypedValue {
    pub fn underlying_type(&self) -> &'static str {
        match self {
            TypedValue::String(_) => "String",
            TypedValue::Bool(_) => "bool",
            TypedValue::SignedInteger(_) => "signed integer",
            TypedValue::Float(_) => "float",
        }
    }

    pub fn is_same_type(&self, other: &TypedValue) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn is_substitutable_type_option(
        this: &Option<TypedValue>,
        other: &Option<TypedValue>,
    ) -> bool {
        match (this, other) {
            (Some(t), Some(o)) => t.is_same_type(o),
            _ => true,
        }
    }

    pub fn display_option(this : Option<Self>) -> String {
        match this {
            Some(v) => v.to_string(),
            None => "None".to_string()
        }
    }
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

macro_rules! impl_try_from {
    ($e:path => $($t:ty),*) => {
        $(impl TryFrom<&TypedValue> for $t {
            type Error = ConfigurationError;

            fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                if let $e(v) = value {
                    Ok(*v as $t)
                } else {
                    Err(ErrorCode::UnexpectedValueType(stringify!($t).to_string(), value.underlying_type().to_string()).into())
                }
            }
        })*
    };
}

impl_try_from!(TypedValue::SignedInteger => i8, i16, i32, i64, isize);
impl_try_from!(TypedValue::Float => f32, f64);
impl_try_from!(TypedValue::Bool => bool);

impl<'a> TryFrom<&'a TypedValue> for &'a str {
    type Error = ConfigurationError;

    fn try_from(value: &'a TypedValue) -> Result<Self, Self::Error> {
        if let TypedValue::String(v) = value {
            Ok(v.as_str())
        } else {
            Err(ErrorCode::UnexpectedValueType(
                "String".to_string(),
                value.underlying_type().to_string(),
            ).into())
        }
    }
}

impl TryFrom<&TypedValue> for String {
    type Error = ConfigurationError;

    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
        if let TypedValue::String(v) = value {
            Ok(v.to_string())
        } else {
            Err(ErrorCode::UnexpectedValueType(
                "String".to_string(),
                value.underlying_type().to_string(),
            ).into())
        }
    }
}
