use std::convert::From;
use crate::{
    configuration::{NodeType, Key},
};
use std::{
    fmt::Display
};

#[derive(Debug)]
pub struct ConfigurationError {
    inner : Box<ErrorImpl>
}

#[derive(Debug)]
struct ErrorImpl {
    error_code : ErrorCode
}

#[derive(Debug)]
pub enum Category {
    ConfigurationAccess,
    ConfigurationMerge,
    SourceCollection,
    SourceDeserialization,
    Other
}

// TODO: Check if it can be done so that keys are not Option
#[derive(Debug)]
pub enum ErrorCode {
    UnexpectedNodeType(Option<Key>, NodeType, NodeType),
    UnexpectedValueType(String, String),
    IndexOutOfRange(Key, usize),
    WrongKeyType(Key, String),
    KeyNotFound(Key, String),
    //
    IncompatibleNodeSubstitution(Option<Key>, NodeType, NodeType),
    IncompatibleValueSubstitution(Option<Key>, String, String),
    //
    IoError(std::io::Error),
    GenericError(Box<dyn std::error::Error>),
    //
    SerdeError(String),
}

impl ConfigurationError {
    pub fn category(&self) -> Category {
        match self.inner.error_code {
            ErrorCode::UnexpectedNodeType(_, _, _) 
            | ErrorCode::UnexpectedValueType(_, _) 
            | ErrorCode::IndexOutOfRange(_, _) 
            | ErrorCode::WrongKeyType(_, _) 
            | ErrorCode::KeyNotFound(_, _) => Category::ConfigurationAccess,
            ErrorCode::IncompatibleNodeSubstitution(_, _, _)  
            | ErrorCode::IncompatibleValueSubstitution(_, _, _) => Category::ConfigurationMerge,
            ErrorCode::IoError(_) => Category::SourceCollection,
            ErrorCode::SerdeError(_) => Category::SourceDeserialization,
            ErrorCode::GenericError(_) => Category::Other
        }
    }
}

// TODO: Finish implementing Error display
impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner.error_code {
            ErrorCode::UnexpectedNodeType(k, exp, act) => { write!(f, "Expected {}, found {} at {:?}.", exp, act, k); } //TODO: Non debug display
            ErrorCode::UnexpectedValueType(_, _) => {}
            ErrorCode::IndexOutOfRange(_, _) => {}
            ErrorCode::WrongKeyType(_, _) => {}
            ErrorCode::KeyNotFound(_, _) => {}
            ErrorCode::IncompatibleNodeSubstitution(_, _, _) => {}
            ErrorCode::IncompatibleValueSubstitution(_, _, _) => {}
            ErrorCode::IoError(_) => {}
            ErrorCode::GenericError(_) => {}
            ErrorCode::SerdeError(_) => {}
        }

        unimplemented!()
    }
}

impl std::error::Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.inner.error_code {
            ErrorCode::IoError(ref e) => Some(e),
            ErrorCode::GenericError(ref e) => Some(e.as_ref()),
            _ => None
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
            inner : Box::new(ErrorImpl {
                error_code : e
            })
        }
    }
}