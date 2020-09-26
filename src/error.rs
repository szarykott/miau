use std::convert::From;
use serde::ser::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigurationAccessError {
    UnexpectedNodeType(&'static str, &'static str),
    UnexpectedValueType(&'static str, &'static str),
    IndexOutOfRange(usize),
    WrongKeyType(String),
    KeyNotFound(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigurationMergeError {
    //TODO: Add more details
    IncompatibleNodeSubstitution,
    IncompatibleValueSubstitution,
}

#[derive(Debug)]
pub enum SourceCollectionError {
    IoError(std::io::Error),
    GenericError(Box<dyn std::error::Error>)
}

impl From<std::io::Error> for SourceCollectionError {
    fn from(e: std::io::Error) -> Self {
        SourceCollectionError::IoError(e)
    }
}

pub enum SourceDeserializationError {
    SerdeError(String),
    GenericError(Box<dyn std::error::Error>)
}