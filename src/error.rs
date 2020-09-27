use std::convert::From;

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
