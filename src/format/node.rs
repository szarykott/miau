use crate::{configuration::Node, error::ConfigurationError, format::ConfigurationDeserializer};

impl ConfigurationDeserializer for Node {
    fn deserialize(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.clone())
    }
}
