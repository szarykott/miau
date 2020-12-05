use std::{convert::Into, default::Default, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigurationInfo {
    pub(crate) source: String,
    pub(crate) format: String,
}

impl ConfigurationInfo {
    pub fn new<T: Into<String>>(source: T, format: T) -> Self {
        ConfigurationInfo {
            source: source.into(),
            format: format.into(),
        }
    }

    pub fn format(&self) -> &str {
        self.format.as_str()
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }
}

impl Default for ConfigurationInfo {
    fn default() -> Self {
        ConfigurationInfo::new("unknown", "unknown")
    }
}

impl fmt::Display for ConfigurationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Source : {}, Format : {}", self.source, self.format)
    }
}
