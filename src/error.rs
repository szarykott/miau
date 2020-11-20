use crate::configuration::{CompoundKey, Key, NodeType};
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
    /// Informs that operation is not valid for given node type e.g descending into value node.
    WrongNodeType(NodeType, NodeType),
    /// Informs that it is not possible to convert value present in configuration to desired type.
    WrongValueType(String, String),
    /// Informs that requested index from array kept in configuration exceeds bounds of the array.
    IndexOutOfRange(usize),
    // TODO: Test this error
    WrongKeyType(String),
    /// Informs that requested key is not present in a configuration.
    KeyNotFound(String),
    /// Informs about errors during merging configuration nodes.
    /// Might occur in circumstances like merging map node with array node.
    BadNodeMerge(NodeType, NodeType),
    /// Informs about different kinds of input/output errors.
    /// Occurs mostly during source collection e.g. reading file or downloading content over network.
    IoError(std::io::Error),
    /// Informs about errors during deserialization.
    /// It covers both external sources and internal structures deserialization.
    DeserializationError(String),
    /// Informs about error attributable to executing invalid operations on empty configuration.
    EmptyConfiguration,
    /// Informs about errors attributable to invalid operation on null value.
    NullValue,
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

    pub fn enrich_with_keys(mut self, keys: &CompoundKey) -> Self {
        if let None = self.inner.path {
            self.inner.path = Some(Vec::new());
        }

        let path = self.inner.path.as_mut().unwrap();

        // this rev() is a nasty hack to ensure ordering
        for key in keys.iter().rev() {
            path.push(key.clone());
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
        writeln!(f, "{}", &self.inner.code)?;

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
            ErrorCode::WrongNodeType(exp, act) => {
                write!(f, "Unexpected node type. Expected {}, got {}", exp, act)
            }
            ErrorCode::WrongValueType(exp, act) => {
                write!(f, "Unexpected value type. Expected {}, got {}", exp, act)
            }
            ErrorCode::IndexOutOfRange(i) => write!(f, "Index {} exceeds bounds of the array.", i),
            ErrorCode::WrongKeyType(k) => write!(f, "Got key of wrong type. Got key {}.", k),
            ErrorCode::KeyNotFound(k) => write!(f, "Unable to find key {}.", k),
            ErrorCode::BadNodeMerge(a, b) => {
                write!(f, "It is forbidden to substitute {} for {}", b, a)
            }
            ErrorCode::IoError(e) => write!(f, "I/O error occurred. {}", e),
            ErrorCode::DeserializationError(e) => write!(f, "Deserialization error occured. {}", e),
            ErrorCode::NullValue => write!(f, "Expected non-null value"),
            ErrorCode::EmptyConfiguration => write!(f, "Expected non-empty configuration"),
            ErrorCode::ParsingError(msg) => write!(f, "Parsing error. {}", msg),
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
