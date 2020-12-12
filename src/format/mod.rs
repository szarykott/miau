use crate::{configuration::ConfigurationTree, error::ConfigurationError};

#[cfg(feature = "ini")]
mod ini;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "msgpack")]
mod msgpack;
#[cfg(feature = "serde_json5")]
mod serde_json5;
#[cfg(feature = "experimental_serde_ron")]
mod serde_ron;
#[cfg(feature = "serde_toml")]
mod serde_toml;
#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "ini")]
pub use ini::Ini;
#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "msgpack")]
pub use msgpack::Msgpack;
#[cfg(feature = "serde_json5")]
pub use serde_json5::Json5;
#[cfg(feature = "experimental_serde_ron")]
pub use serde_ron::Ron;
#[cfg(feature = "serde_toml")]
pub use serde_toml::Toml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;

/// Represents data format
pub trait Format {
    /// Transforms raw data into `ConfigurationTree`
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError>;
    /// Describes this `Format`.
    fn describe(&self) -> String;
}

impl<T> Format for T
where
    T: Fn(Vec<u8>) -> Result<ConfigurationTree, ConfigurationError>,
{
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        self(input)
    }

    fn describe(&self) -> String {
        "custom".into()
    }
}

/// Utility function to create `json` format deserializer.
#[cfg(feature = "json")]
pub fn json() -> Json {
    Json::default()
}

/// Utility function to create `yaml` format deserializer.
#[cfg(feature = "yaml")]
pub fn yaml() -> Yaml {
    Yaml::default()
}

/// Utility function to create `toml` format deserializer.
#[cfg(feature = "serde_toml")]
pub fn toml() -> Toml {
    Toml::default()
}

/// Utility function to create `json5` format deserializer.
#[cfg(feature = "serde_json5")]
pub fn json5() -> Json5 {
    Json5::default()
}

/// Utility function to create `ron` format deserializer.
///
/// Uses external deserializer so it is only as good as it is.
#[cfg(feature = "experimental_serde_ron")]
pub fn ron() -> Ron {
    Ron::default()
}

/// Utility function to create `message pack` format deserializer.
#[cfg(feature = "msgpack")]
pub fn msgpack() -> Msgpack {
    Msgpack::default()
}

/// Utility function to create `ini` format deserializer.
#[cfg(feature = "ini")]
pub fn ini() -> Ini {
    Ini::default()
}
