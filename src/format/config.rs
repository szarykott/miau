use crate::{
    configuration::{Configuration, MergedConfiguration, Node},
    error::ConfigurationError,
    format::ConfigurationDeserializer,
};

impl ConfigurationDeserializer for Configuration {
    fn deserialize(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.clone().merge_owned()?.root)
    }
}

impl ConfigurationDeserializer for MergedConfiguration {
    fn deserialize(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.root.clone())
    }
}
