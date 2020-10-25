use crate::{
    configuration::{Configuration, Node},
    error::ConfigurationError,
    format::Transformer,
};

impl Transformer for Node {
    fn transform(&self, _input: String) -> Result<Configuration, ConfigurationError> {
        Ok(self.clone().into())
    }
}
