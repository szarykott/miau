use super::{ConfigurationInfo, ConfigurationTree};
use std::convert::From;

/// Holds informations about configuration along with configuration tree root.
#[derive(Debug, Clone)]
pub struct ConfigurationDefinition {
    pub(crate) info: ConfigurationInfo,
    pub(crate) root: ConfigurationTree,
}

/// Borrowed version of [`ConfigurationDefinition`]
#[derive(Debug)]
pub struct ConfigurationDefinitionLens<'config> {
    pub(crate) info: &'config ConfigurationInfo,
    pub(crate) node: Option<&'config ConfigurationTree>,
}

impl ConfigurationDefinition {
    /// Creates new instance of [`ConfigurationDefinition`]
    pub fn new(info: ConfigurationInfo, root: ConfigurationTree) -> Self {
        ConfigurationDefinition { info, root }
    }
}

impl<'config> ConfigurationDefinitionLens<'config> {
    /// Returns new [`ConfigurationDefinitionLens`] with `func` applied to node.
    ///
    /// It is mainly meant as a way to abstract away lensing into further parts of coniguration tree.
    pub fn mutate<F>(&self, func: F) -> Self
    where
        F: Fn(&'config ConfigurationTree) -> Option<&'config ConfigurationTree>,
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
