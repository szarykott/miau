use crate::error::ConfigurationError;
/// Represents object from which configuration can be read.
pub trait ConfigurationRead<'config, T, K> {
    /// Retrieves value stored in configuration under given `keys`.
    ///
    /// If no value is found or key transformation fails `None` is returned.
    /// [`get_result`](Self::get_result) provides more insight into root cause of error.
    ///
    /// # Example
    /// TODO: add example
    fn get(&'config self, keys: K) -> Option<T> {
        self.get_result(keys).ok().unwrap_or_default()
    }

    /// Retrieves value stored in `ConfigurationTree` under given `keys`.
    ///
    /// If key transformation fails error is returned. Value is returned if found, `None` otherwise.
    ///
    /// # Example
    /// TODO: add example
    fn get_result(&'config self, keys: K) -> Result<Option<T>, ConfigurationError>;
}
