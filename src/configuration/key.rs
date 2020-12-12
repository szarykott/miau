use crate::{error::ConfigurationError, parsing};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;
use std::{convert::From, fmt};

///Multikey for configuration
///
///Consists of multiple [keys](Key)
///
///It is used indirectly in many functions reading from configuration.
///Usually it should not be constructed directly.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct CompoundKey(Vec<Key>);

///Configuration key
///
///It comes in two flavours - for arrays and maps.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Key {
    ///Variant that is used to index into arrays
    Array(usize),
    ///Variant that is used as a key in maps
    Map(String),
}

impl CompoundKey {
    /// Constructs new instance of `CompoundKey`
    pub fn new(keys: Vec<Key>) -> Self {
        CompoundKey(keys)
    }
}

impl Key {
    /// Unwraps underlying string in map variant
    ///
    /// # Panics
    /// Panics if `Key` variant is `Array`
    pub fn unwrap_map(&self) -> String {
        match self {
            Key::Array(_) => panic!("Expected key to be map key!"),
            Key::Map(s) => s.clone(),
        }
    }
    /// Unwraps underlying usize in map variant
    ///
    /// # Panics
    /// Panics if `Key` variant is `Map`
    pub fn unwrap_array(&self) -> usize {
        match self {
            Key::Array(i) => *i,
            Key::Map(_) => panic!("Expected key to be array key!"),
        }
    }
}

impl Deref for CompoundKey {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Key>> for CompoundKey {
    fn from(keys: Vec<Key>) -> Self {
        CompoundKey::new(keys)
    }
}

impl TryFrom<&str> for CompoundKey {
    type Error = ConfigurationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parsing::str_to_key(value)
            .map_err(|e| e.enrich_with_context(format!("Parsing key `{}` failed", value)))
    }
}

impl TryFrom<String> for CompoundKey {
    type Error = ConfigurationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        parsing::str_to_key(value.as_ref())
            .map_err(|e| e.enrich_with_context(format!("Parsing key `{}` failed", value)))
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Array(i) => write!(f, "[{}]", i),
            Key::Map(k) => write!(f, "{}", k),
        }
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Key::Map(v)
    }
}

impl From<&str> for Key {
    fn from(v: &str) -> Self {
        Key::Map(v.to_string())
    }
}

macro_rules! impl_key_from {
    ($($t:ty),*) => {
        $(impl From<$t> for Key {
            fn from(v: $t) -> Self {
                Key::Array(v as usize)
            }
        })*
    };
}

impl_key_from!(u8, u16, u32, u64, usize);
