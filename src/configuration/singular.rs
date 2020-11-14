use crate::{
    configuration::{node::Node, CompoundKey, Lens, Value},
    error::ConfigurationError,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SingularConfiguration {
    pub(crate) root: Node,
}

impl SingularConfiguration {
    pub fn get<'config, T, S>(&'config self, keys: S) -> Option<T>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey>,
    {
        self.root.get_option(&keys.try_into().ok()?)
    }

    pub fn get_result<'config, T, S>(
        &'config self,
        keys: S,
    ) -> Result<Option<T>, ConfigurationError>
    where
        T: TryFrom<&'config Value, Error = ConfigurationError>,
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        self.root.get_result(&keys.try_into()?)
    }

    pub fn try_into<T: DeserializeOwned>(self) -> Result<T, ConfigurationError> {
        Node::try_into::<T>(&self.root)
    }

    pub fn try_lens<'config, S>(&'config self, keys: S) -> Result<Lens<'config>, ConfigurationError>
    where
        S: TryInto<CompoundKey, Error = ConfigurationError>,
    {
        Ok(Lens::new(self.root.descend_many(&keys.try_into()?)?))
    }
}

impl From<Node> for SingularConfiguration {
    fn from(node: Node) -> Self {
        SingularConfiguration { root: node }
    }
}
