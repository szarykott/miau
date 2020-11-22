use super::{ConfigurationInfo, ConfigurationNode};
use std::convert::From;

#[derive(Debug, Clone)]
pub struct ConfigurationDefinition {
    pub(crate) info: ConfigurationInfo,
    pub(crate) root: ConfigurationNode,
}

#[derive(Debug)]
pub struct ConfigurationDefinitionLens<'config> {
    pub(crate) info: &'config ConfigurationInfo,
    pub(crate) node: Option<&'config ConfigurationNode>,
}

impl ConfigurationDefinition {
    pub fn new(info: ConfigurationInfo, root: ConfigurationNode) -> Self {
        ConfigurationDefinition { info, root }
    }
}

impl<'config> ConfigurationDefinitionLens<'config> {
    pub fn mutate<F>(&self, func: F) -> Self
    where
        F: Fn(&'config ConfigurationNode) -> Option<&'config ConfigurationNode>,
    {
        ConfigurationDefinitionLens {
            info: self.info,
            node: self.node.and_then(|node| func(node)),
        }
    }
}

impl<'config> From<&'config ConfigurationDefinition> for ConfigurationDefinitionLens<'config> {
    fn from(root_def: &'config ConfigurationDefinition) -> Self {
        ConfigurationDefinitionLens {
            info: &root_def.info,
            node: Some(&root_def.root),
        }
    }
}
