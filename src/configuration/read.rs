use crate::error::ConfigurationError;
/// Represents object from which configuration can be read.
pub trait ConfigurationRead<'config, T, K> {
    /// Retrieves value stored in configuration under given `keys`.
    ///
    /// If no value is found or key transformation fails `None` is returned.
    /// [`get_result`](Self::get_result) provides more insight into root cause of error.
    ///
    /// # Example
    ///```rust
    /// //you need to have ConfigurationRead in scope
    ///use miau::configuration::{Configuration, ConfigurationRead};
    ///
    ///let configuration = Configuration::default(); //  aka empty
    ///let word: Option<String> = configuration.get("word");
    ///assert_eq!(None, word);
    ///```
    fn get(&'config self, keys: K) -> Option<T> {
        self.get_result(keys).ok().unwrap_or_default()
    }

    /// Retrieves value stored in `ConfigurationTree` under given `keys`.
    ///
    /// If key transformation fails error is returned. Value is returned if found, `None` otherwise.
    ///
    /// # Example
    ///```rust
    /// //you need to have ConfigurationRead in scope
    ///use miau::configuration::{Configuration, ConfigurationRead};
    ///use miau::error::ConfigurationError;
    ///
    ///let configuration = Configuration::default(); //  aka empty
    ///let word: Result<Option<String>, ConfigurationError> = configuration.get_result("word");
    ///match word {
    ///     Ok(maybe_word) => {
    ///         assert_eq!(None, maybe_word);
    ///     },
    ///     Err(e) => println!("Oh no! {}", e)
    ///};
    ///```
    fn get_result(&'config self, keys: K) -> Result<Option<T>, ConfigurationError>;
}
