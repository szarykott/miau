use crate::{
    configuration::{node, node::ConfigurationNode, CompoundKey, Value},
    error::{ConfigurationError, ErrorCode},
};
use std::convert::TryFrom;

pub fn get_result_internal<'a, T>(
    nodes: &'a [ConfigurationNode],
    keys: CompoundKey,
) -> Result<Option<T>, ConfigurationError>
where
    T: TryFrom<&'a Value, Error = ConfigurationError>,
{
    for candidate in nodes.iter().rev() {
        if let result @ Ok(_) = candidate.get_result_internal::<T>(&keys) {
            return result;
        }
    }

    Ok(None)
}

pub fn get_result_option_internal<'config, T>(
    nodes: &'config [Option<&'config ConfigurationNode>],
    keys: &CompoundKey,
) -> Result<Option<T>, ConfigurationError>
where
    T: TryFrom<&'config Value, Error = ConfigurationError>,
{
    for candidate in nodes.iter().rev() {
        if let Some(node) = candidate {
            if let result @ Ok(_) = node.get_result_internal::<T>(&keys) {
                return result;
            }
        }
    }

    Ok(None)
}

pub fn merge_cloned<'config>(
    nodes: impl Iterator<Item = &'config ConfigurationNode>,
) -> Result<ConfigurationNode, ConfigurationError> {
    merge_owned(nodes.cloned())
}

pub fn merge_owned(
    mut nodes: impl Iterator<Item = ConfigurationNode>,
) -> Result<ConfigurationNode, ConfigurationError> {
    match nodes.next() {
        Some(node) => nodes.try_fold(node, |acc, next| node::merge(acc, next)),
        None => {
            let error: ConfigurationError = ErrorCode::EmptyConfiguration.into();
            Err(error.enrich_with_context("Failed to merge configurations"))
        }
    }
}
