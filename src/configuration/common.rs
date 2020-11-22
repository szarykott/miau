use crate::{
    configuration::{node, node::ConfigurationNode, CompoundKey, Value},
    error::{ConfigurationError, ErrorCode},
};
use std::{convert::TryFrom, iter::DoubleEndedIterator};

pub fn get_result_internal<'config, T>(
    nodes: impl DoubleEndedIterator<Item = &'config ConfigurationNode>,
    keys: &CompoundKey,
) -> Result<Option<T>, ConfigurationError>
where
    T: TryFrom<&'config Value, Error = ConfigurationError>,
{
    for candidate in nodes.rev() {
        if let result @ Ok(_) = candidate.get_result_internal::<T>(keys) {
            return result;
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
        Some(node) => nodes.try_fold(node, node::merge),
        None => {
            let error: ConfigurationError = ErrorCode::EmptyConfiguration.into();
            Err(error.enrich_with_context("Failed to merge configurations"))
        }
    }
}
