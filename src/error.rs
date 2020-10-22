use crate::configuration::{Key, NodeType};
use serde::de;
use std::{convert::From, fmt::Display};

#[derive(Debug)]
pub struct ConfigurationError {
    inner: Box<ErrorImpl>,
}

#[derive(Debug)]
struct ErrorImpl {
    code: ErrorCode,
    path: Vec<Key>,
}

struct KeyVec<'v>(&'v [Key]);

impl<'v> Display for KeyVec<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for key in self.0.iter() {
            write!(f, " {} ", key)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Category {
    ConfigurationAccess,
    ConfigurationMerge,
    SourceCollection,
    SourceDeserialization,
    Other,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::ConfigurationAccess => write!(f, "configuration access"),
            Category::ConfigurationMerge => write!(f, "configuration merge"),
            Category::SourceCollection => write!(f, "source collection"),
            Category::SourceDeserialization => write!(f, "source deserialization"),
            Category::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    UnexpectedNodeType(NodeType, NodeType),
    UnexpectedValueType(String, String),
    IndexOutOfRange(usize),
    WrongKeyType(String),
    KeyNotFound(String),
    IncompatibleNodeSubstitution(NodeType, NodeType),
    IncompatibleValueSubstitution(String, String),
    IoError(std::io::Error),
    GenericError(Box<dyn std::error::Error>),
    SerdeError(String),
    MissingValue,
}

impl ConfigurationError {
    pub fn category(&self) -> Category {
        match self.inner.code {
            ErrorCode::UnexpectedNodeType(_, _)
            | ErrorCode::UnexpectedValueType(_, _)
            | ErrorCode::IndexOutOfRange(_)
            | ErrorCode::WrongKeyType(_)
            | ErrorCode::MissingValue
            | ErrorCode::KeyNotFound(_) => Category::ConfigurationAccess,
            ErrorCode::IncompatibleNodeSubstitution(_, _)
            | ErrorCode::IncompatibleValueSubstitution(_, _) => Category::ConfigurationMerge,
            ErrorCode::IoError(_) => Category::SourceCollection,
            ErrorCode::SerdeError(_) => Category::SourceDeserialization,
            ErrorCode::GenericError(_) => Category::Other,
        }
    }

    pub fn inner(&self) -> &ErrorCode {
        &self.inner.code
    }

    pub fn enrich_with_key(mut self, key: Key) -> Self {
        self.inner.path.push(key);
        self
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error category : {}, ", self.category())?;

        match self.category() {
            Category::ConfigurationAccess
            | Category::ConfigurationMerge
            | Category::SourceDeserialization => {
                write!(f, "path : {}, ", KeyVec(&self.inner.path))?;
            }
            _ => {}
        }

        match &self.inner.code {
            ErrorCode::UnexpectedNodeType(exp, act) => {
                write!(f, "unexpected node type. Expected {}, got {}.", exp, act)
            }
            ErrorCode::UnexpectedValueType(exp, act) => {
                write!(f, "unexpected value type. Expected {}, got {}.", exp, act)
            }
            ErrorCode::IndexOutOfRange(i) => write!(f, "index {} exceeds bounds of the array.", i),
            ErrorCode::WrongKeyType(k) => write!(f, "got key of wrong type. Got key {}.", k),
            ErrorCode::KeyNotFound(k) => write!(f, "unable to find key {}.", k),
            ErrorCode::IncompatibleNodeSubstitution(a, b) => {
                write!(f, "it is forbidden to substitute {} for {}.", a, b)
            }
            ErrorCode::IncompatibleValueSubstitution(a, b) => {
                write!(f, "it is forbidden to substitute {} for {}.", a, b)
            }
            ErrorCode::IoError(e) => write!(f, "IO error occurred. Error : {}.", e),
            ErrorCode::GenericError(e) => write!(f, "an error occured : {}.", e),
            ErrorCode::SerdeError(e) => {
                write!(f, "serialization or deserialization error occured : {}.", e)
            }
            ErrorCode::MissingValue => write!(f, "missing a value."),
        }
    }
}

impl std::error::Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner.code {
            ErrorCode::IoError(ref e) => Some(e),
            ErrorCode::GenericError(ref e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigurationError {
    fn from(e: std::io::Error) -> Self {
        ConfigurationError::from(ErrorCode::IoError(e))
    }
}

impl From<ErrorCode> for ConfigurationError {
    fn from(e: ErrorCode) -> Self {
        ConfigurationError {
            inner: Box::new(ErrorImpl {
                code: e,
                path: Vec::new(),
            }),
        }
    }
}

impl de::Error for ConfigurationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ConfigurationError::from(ErrorCode::SerdeError(msg.to_string()))
    }
}
