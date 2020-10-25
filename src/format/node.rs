use crate::{configuration::Node, error::ConfigurationError, format::Transformer};

impl Transformer for Node {
    fn transform(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.clone())
    }
}
