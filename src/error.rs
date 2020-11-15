use crate::configuration::{Key, NodeType};
use serde::de;
use std::{convert::From, fmt::Display, ops::Deref};

#[derive(Debug)]
pub struct ConfigurationError {
    inner: Box<ErrorImpl>,
}

#[derive(Debug)]
pub struct ErrorImpl {
    code: ErrorCode,
    context: Option<Vec<String>>,
    path: Option<Vec<Key>>,
}

// TODO: Rethink errors in here!
#[derive(Debug)]
pub enum ErrorCode {
    UnexpectedNodeType(NodeType, NodeType),
    UnexpectedValueType(String, String),
    IndexOutOfRange(usize),
    WrongKeyType(String),
    KeyNotFound(String),
    /// Informs about errors during merging configuration nodes.
    /// Might occur in circumstances like merging map node with array node.
    BadMerge(NodeType, NodeType),
    /// Informs about different kinds of input/output errors.
    /// Occurs mostly during source collection e.g. reading file or downloading content over network.
    IoError(std::io::Error),
    /// Informs about errors during deserialization.
    /// It covers both external sources and internal structures deserialization.
    DeserializationError(String),
    MissingValue,
    /// Informs about parsing error that occured.
    ParsingError(String),
}

impl ConfigurationError {
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

impl ErrorImpl {
    pub fn get_code(&self) -> &ErrorCode {
        &self.code
    }

    pub fn get_path(&self) -> Option<&[Key]> {
        if let Some(ref path) = self.path {
            Some(path)
        } else {
            None
        }
    }

    pub fn get_context(&self) -> Option<&[String]> {
        if let Some(ref context) = self.context {
            Some(context)
        } else {
            None
        }
    }
}

struct KeyVec<'v>(&'v [Key]);

impl<'v> Display for KeyVec<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for key in self.0.iter().rev() {
            if first {
                write!(f, "{}", key)?;
                first = false;
                continue;
            }
            write!(f, "-->{}", key)?;
        }
        Ok(())
    }
}

impl Deref for ConfigurationError {
    type Target = ErrorImpl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// TODO: Make this display messages in one line and create additional wrapper PrettyDisplay so that error message is nice to all users
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.inner.code)?;

        if let Some(ref path) = self.inner.path {
            writeln!(f, "Path : {}", KeyVec(path))?;
        }

        if let Some(ref context) = self.inner.context {
            writeln!(f, "Context: ")?;
            for msg in context.iter() {
                writeln!(f, "\t{}", msg)?;
            }
        }

        Ok(())
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            ErrorCode::BadMerge(a, b) => {
                writeln!(f, "It is forbidden to substitute {} for {}.", b, a)
            }
            ErrorCode::IoError(e) => writeln!(f, "I/O error occurred. {}.", e),
            ErrorCode::DeserializationError(e) => {
                writeln!(f, "Deserialization error occured : {}.", e)
            }
            ErrorCode::MissingValue => writeln!(f, "Missing a value."),
            ErrorCode::ParsingError(msg) => writeln!(f, "Parsing error. {}", msg),
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
