use std::convert::From;
use crate::{
    configuration::{NodeType, Key},
};

#[derive(Debug)]
pub struct Error {
    inner : Box<ErrorImpl>
}

#[derive(Debug)]
struct ErrorImpl {
    error_code : ErrorCode
}

#[derive(Debug)]
pub enum Category {
    Generic,
    ConfigurationAccess,
    ConfigurationMerge,
    SourceCollection,
    SourceDeserialization
}

#[derive(Debug)]
pub enum ErrorCode {
    UnexpectedNodeType(Key, NodeType, NodeType),
    UnexpectedValueType(String, String),
    IndexOutOfRange(Key, usize),
    WrongKeyType(Key, String),
    KeyNotFound(Key, String),
    //
    IncompatibleNodeSubstitution(Key, NodeType, NodeType),
    IncompatibleValueSubstitution(Key, &'static str, &'static str),
    //
    IoError(std::io::Error),
    GenericError(Box<dyn std::error::Error>),
    //
    SerdeError(String),
}

impl Error {
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
            ErrorCode::GenericError(_) => Category::Generic
        }
    }
}

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
pub enum SourceError {
    CollectionError(SourceCollectionError),
    DeserializationError(SourceDeserializationError),
}

impl From<SourceCollectionError> for SourceError {
    fn from(e: SourceCollectionError) -> Self {
        SourceError::CollectionError(e)
    }
}

impl From<SourceDeserializationError> for SourceError {
    fn from(e: SourceDeserializationError) -> Self {
        SourceError::DeserializationError(e)
    }
}

#[derive(Debug)]
pub enum SourceCollectionError {
    IoError(std::io::Error),
    GenericError(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for SourceCollectionError {
    fn from(e: std::io::Error) -> Self {
        SourceCollectionError::IoError(e)
    }
}

#[derive(Debug)]
pub enum SourceDeserializationError {
    SerdeError(String),
    GenericError(Box<dyn std::error::Error>),
}
