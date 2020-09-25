use serde::{Deserialize, Serialize};
use std::convert::From;

pub type CompoundKey = Vec<Key>;

#[macro_export]
macro_rules! compound_key {
    [$($val:expr),*] => {{
        let mut ck : $crate::configuration::CompoundKey = Vec::new();
        $(ck.push($crate::configuration::Key::from($val));)*
        ck
    }};
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Key {
    Array(usize),
    Map(String),
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
