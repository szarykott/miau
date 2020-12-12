use crate::{
    configuration::{tree, CompoundKey, ConfigurationTree, Value},
    error::{ConfigurationError, ErrorCode},
};
use std::{convert::TryFrom, iter::DoubleEndedIterator};

pub fn get_result_internal<'config, T>(
    nodes: impl DoubleEndedIterator<Item = &'config ConfigurationTree>,
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
    nodes: impl Iterator<Item = &'config ConfigurationTree>,
) -> Result<ConfigurationTree, ConfigurationError> {
    merge_owned(nodes.cloned())
}

pub fn merge_owned(
    mut nodes: impl Iterator<Item = ConfigurationTree>,
) -> Result<ConfigurationTree, ConfigurationError> {
    match nodes.next() {
        Some(node) => nodes.try_fold(node, tree::merge),
        None => {
            let error: ConfigurationError = ErrorCode::EmptyConfiguration.into();
            Err(error.enrich_with_context("Failed to merge configurations"))
        }
    }
}
