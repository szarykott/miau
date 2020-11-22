use crate::{configuration::ConfigurationNode, error::ConfigurationError};

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
#[cfg(feature = "experimental_xml")]
mod xml;
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
#[cfg(feature = "experimental_xml")]
pub use xml::Xml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;

pub trait Format {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError>;
    fn describe(&self) -> String;
}

impl<T> Format for T
where
    T: Fn(Vec<u8>) -> Result<ConfigurationNode, ConfigurationError>,
{
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationNode, ConfigurationError> {
        self(input)
    }

    fn describe(&self) -> String {
        "custom".into()
    }
}

#[cfg(feature = "json")]
pub fn json() -> Json {
    Json::default()
}

#[cfg(feature = "yaml")]
pub fn yaml() -> Yaml {
    Yaml::default()
}

#[cfg(feature = "serde_toml")]
pub fn toml() -> Toml {
    Toml::default()
}

#[cfg(feature = "serde_json5")]
pub fn json5() -> Json5 {
    Json5::default()
}

/// Experimental RON deserializer.
/// Might require hacks to work.
#[cfg(feature = "experimental_serde_ron")]
pub fn ron() -> Ron {
    Ron::default()
}

#[cfg(feature = "msgpack")]
pub fn msgpack() -> Msgpack {
    Msgpack::default()
}

#[cfg(feature = "ini")]
pub fn ini() -> Ini {
    Ini::default()
}

#[cfg(feature = "experimental_xml")]
pub fn xml() -> Xml {
    Xml::default()
}
