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

    pub fn display_option(this: Option<Self>) -> String {
        match this {
            Some(v) => v.to_string(),
            None => "None".to_string(),
        }
    }

    //TODO: Is try_convert_option really needed?
    // pub fn try_convert_option(
    //     prev: Option<TypedValue>,
    //     next: Option<TypedValue>,
    // ) -> Result<Option<TypedValue>, ConfigurationError> {
    //     match (prev, next) {
    //         (None, s) => Ok(s),
    //         (Some(_), None) => Ok(None),
    //         (Some(pv), Some(nv)) => match (pv, nv) {
    //             (TypedValue::String(_), nv @ TypedValue::String(_)) => Ok(next),
    //             (TypedValue::String(pv), TypedValue::Bool(_)) => {
    //                 if let Ok(_) = pv.parse::<bool>() {
    //                     Ok(Some(nv))
    //                 } else {
    //                     Err(ErrorCode::IncompatibleValueSubstitution(
    //                         None,
    //                         TypedValue::display_option(prev),
    //                         TypedValue::display_option(next),
    //                     )
    //                     .into())
    //                 }
    //             }
    //             (TypedValue::String(_), TypedValue::SignedInteger(_)) => {}
    //             (TypedValue::String(_), TypedValue::Float(_)) => {}
    //             (TypedValue::Bool(_), TypedValue::String(_)) => {}
    //             (TypedValue::Bool(_), TypedValue::Bool(_)) => Ok(next),
    //             (TypedValue::Bool(_), TypedValue::SignedInteger(_)) => {}
    //             (TypedValue::Bool(_), TypedValue::Float(_)) => {}
    //             (TypedValue::SignedInteger(_), TypedValue::String(_)) => {}
    //             (TypedValue::SignedInteger(_), TypedValue::Bool(_)) => {}
    //             (TypedValue::SignedInteger(_), TypedValue::SignedInteger(_)) => Ok(next),
    //             (TypedValue::SignedInteger(_), TypedValue::Float(_)) => {}
    //             (TypedValue::Float(_), TypedValue::String(_)) => {}
    //             (TypedValue::Float(_), TypedValue::Bool(_)) => {}
    //             (TypedValue::Float(_), TypedValue::SignedInteger(_)) => {}
    //             (TypedValue::Float(_), TypedValue::Float(_)) => Ok(next),
    //         },
    //     }
    // }
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

// TODO: Fix those implementations so that they are as permissive as possible
// e.g everyting is convertible to string (not &str unfortunately)
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


impl TryFrom<&TypedValue> for String {
    type Error = ConfigurationError;

    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
        if let TypedValue::String(v) = value {
            Ok(v.to_string())
        } else {
            Err(ErrorCode::UnexpectedValueType(
                "String".to_string(),
                value.underlying_type().to_string(),
            )
            .into())
        }
    }
}
