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
    context: Option<Vec<String>>,
    path: Option<Vec<Key>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Category {
    ConfigurationAccess,
    ConfigurationMerge,
    SourceCollection,
    /// Category of errors that occur during deserialization.
    /// It covers external sources and internal library's structures deserialization.
    Deserialization,
    Other,
}

// TODO: Rethink errors in here!
// Maybe split them per module and only aggregate here?
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
    DeserializationError(String),
    MissingValue,
    ParsingError(String),
}

impl ConfigurationError {
    pub fn category(&self) -> Category {
        match self.inner.code {
            ErrorCode::UnexpectedNodeType(_, _)
            | ErrorCode::UnexpectedValueType(_, _)
            | ErrorCode::IndexOutOfRange(_)
            | ErrorCode::WrongKeyType(_)
            | ErrorCode::MissingValue
            | ErrorCode::ParsingError(_)
            | ErrorCode::KeyNotFound(_) => Category::ConfigurationAccess,
            ErrorCode::IncompatibleNodeSubstitution(_, _)
            | ErrorCode::IncompatibleValueSubstitution(_, _) => Category::ConfigurationMerge,
            ErrorCode::IoError(_) => Category::SourceCollection,
            ErrorCode::DeserializationError(_) => Category::Deserialization,
        }
    }

    pub fn inner(&self) -> &ErrorCode {
        &self.inner.code
    }

    pub fn enrich_with_context<T: Into<String>>(mut self, message: T) -> Self {
        match self.inner.context {
            Some(ref mut context) => context.push(message.into()),
            None => {
                self.inner.context = Some(vec![message.into()]);
            }
        }
        self
    }

    pub fn enrich_with_key(mut self, key: Key) -> Self {
        match self.inner.path {
            Some(ref mut path) => {
                path.push(key);
            }
            None => {
                self.inner.path = Some(vec![key]);
            }
        }
        self
    }
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

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::ConfigurationAccess => write!(f, "configuration access"),
            Category::ConfigurationMerge => write!(f, "configuration merge"),
            Category::SourceCollection => write!(f, "source collection"),
            Category::Deserialization => write!(f, "deserialization"),
            Category::Other => write!(f, "other"),
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error category : {}", self.category())?;

        match self.category() {
            Category::ConfigurationAccess
            | Category::ConfigurationMerge
            | Category::Deserialization => {
                if let Some(ref path) = self.inner.path {
                    writeln!(f, "Path : {}", KeyVec(path))?;
                }
            }
            _ => {}
        }

        writeln!(f, "Cause: ")?;

        if let Some(ref context) = self.inner.context {
            for msg in context.iter() {
                writeln!(f, "{}", msg)?;
            }
        }

        match &self.inner.code {
            ErrorCode::UnexpectedNodeType(exp, act) => {
                writeln!(f, "Unexpected node type. Expected {}, got {}.", exp, act)
            }
            ErrorCode::UnexpectedValueType(exp, act) => {
                writeln!(f, "Unexpected value type. Expected {}, got {}.", exp, act)
            }
            ErrorCode::IndexOutOfRange(i) => {
                writeln!(f, "Index {} exceeds bounds of the array.", i)
            }
            ErrorCode::WrongKeyType(k) => writeln!(f, "Got key of wrong type. Got key {}.", k),
            ErrorCode::KeyNotFound(k) => writeln!(f, "Unable to find key {}.", k),
            ErrorCode::IncompatibleNodeSubstitution(a, b) => {
                writeln!(f, "It is forbidden to substitute {} for {}.", a, b)
            }
            ErrorCode::IncompatibleValueSubstitution(a, b) => {
                writeln!(f, "It is forbidden to substitute {} for {}.", a, b)
            }
            ErrorCode::IoError(e) => writeln!(f, "IO error occurred. Error : {}.", e),
            ErrorCode::DeserializationError(e) => {
                writeln!(f, "Deserialization error occured : {}.", e)
            }
            ErrorCode::MissingValue => writeln!(f, "Missing a value."),
            ErrorCode::ParsingError(msg) => writeln!(f, "Error while parsing : {}", msg),
        }
    }
}

impl std::error::Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner.code {
            ErrorCode::IoError(ref e) => Some(e),
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
                context: None,
                path: None,
            }),
        }
    }
}

impl de::Error for ConfigurationError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ConfigurationError::from(ErrorCode::DeserializationError(msg.to_string()))
    }
}
