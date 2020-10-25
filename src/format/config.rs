use crate::{
    configuration::{Configuration, MergedConfiguration, Node},
    error::ConfigurationError,
    format::Transformer,
};

impl Transformer for Configuration {
    fn transform(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.clone().merge_owned()?.root)
    }
}

impl Transformer for MergedConfiguration {
    fn transform(&self, _input: String) -> Result<Node, ConfigurationError> {
        Ok(self.root.clone())
    }
}
