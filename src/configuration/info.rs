use std::{convert::Into, default::Default, fmt};

/// Holds information about configuration.
///
/// Each configuration source and format can populate it with arbitrary data.
/// For instance, file source may store path to a file and its format.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigurationInfo {
    pub(crate) source: String,
    pub(crate) format: String,
}

impl ConfigurationInfo {
    /// Creates new instance of `ConfigurationInfo`.
    pub fn new<T: Into<String>>(source: T, format: T) -> Self {
        ConfigurationInfo {
            source: source.into(),
            format: format.into(),
        }
    }

    /// Returns information about configuration format.
    ///
    /// Usually it will be data format information like `json`, `yaml`.
    /// However, arbitrary data can be stored as a configuration format,
    /// thus no assumption about cotents of format string should be made.
    pub fn format(&self) -> &str {
        self.format.as_str()
    }

    /// Returns information about configuration source.
    ///
    /// Depending on a source this information can contain file path, url or simple information like 'environment'.
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
        write!(f, "source : {}, format : {}", self.source, self.format)
    }
}
