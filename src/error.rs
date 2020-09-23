#[derive(Debug)]
pub enum ConfigurationAccessError {
    UnexpectedNodeType(&'static str, &'static str),
    UnexpectedValueType(&'static str, &'static str),
    IndexOutOfRange(usize),
    WrongKeyType(String),
    KeyNotFound(String),
}

#[derive(Debug)]
pub enum ConfigurationMergeError {
    //TODO: Add more details
    IncompatibleNodeSubstitution,
    IncompatibleValueSubstitution
}
