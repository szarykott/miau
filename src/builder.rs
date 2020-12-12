use crate::{
    configuration::Configuration,
    error::ConfigurationError,
    format::Format,
    provider::{AsyncProvider, Provider, ProviderStruct},
    source::{AsyncSource, Source},
};
use std::default::Default;

/// Synchronous configuration builder.
///
/// Owns all sources passed to it and is capable of creating Configuration object.
pub struct ConfigurationBuilder<'provider> {
    sources: Vec<Box<dyn Provider + 'provider>>,
}

impl<'provider> Default for ConfigurationBuilder<'provider> {
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

impl<'provider> ConfigurationBuilder<'provider> {
    /// Creates new builder.
    ///
    /// This function is used in Default trait implementation.
    ///```rust
    ///use miau::builder::ConfigurationBuilder;
    ///
    ///let builder = ConfigurationBuilder::new();
    ///```
    pub fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    /// Adds new source and format to builder.
    ///
    /// It only accepts synchronous sources.
    ///```rust
    ///use miau::builder::ConfigurationBuilder;
    ///use miau::source::FileSource;
    ///use miau::format;
    ///
    ///let mut builder = ConfigurationBuilder::default();
    ///builder.add(FileSource::from_path("./a/path/to/file.json"), format::json());
    ///```
    pub fn add<S, D>(&mut self, source: S, format: D) -> &mut ConfigurationBuilder<'provider>
    where
        S: Source + 'provider,
        D: Format + 'provider,
    {
        self.add_provider(ProviderStruct::synchronous(source, format));
        self
    }

    /// Adds new provider to builder.
    ///
    /// It only accepts synchronous providers.
    ///```rust
    ///use miau::builder::ConfigurationBuilder;
    ///use miau::provider::EnvironmentProvider;
    ///use miau::format;
    ///
    ///let mut builder = ConfigurationBuilder::default();
    ///builder.add_provider(EnvironmentProvider::default());
    ///```
    pub fn add_provider<P>(&mut self, provider: P) -> &mut ConfigurationBuilder<'provider>
    where
        P: Provider + 'provider,
    {
        self.sources.push(Box::new(provider));
        self
    }

    /// Adds new source and format to builder.
    ///
    /// Similar to [add](Self::add()), but only accepts asynchronous providers.
    /// **Operation is consuming**, asynchronous version of builder is returned.
    pub fn add_async<S, D>(self, source: S, format: D) -> AsyncConfigurationBuilder<'provider>
    where
        S: AsyncSource + Send + Sync + 'provider,
        D: Format + Send + Sync + 'provider,
    {
        self.add_provider_async(ProviderStruct::asynchronous(source, format))
    }

    /// Adds new provider to builder.
    ///
    /// Similar to [add_provider](Self::add_provider()), but only accepts asynchronous providers.
    /// **Operation is consuming**, asynchronous version of builder is return
    pub fn add_provider_async<P>(self, provider: P) -> AsyncConfigurationBuilder<'provider>
    where
        P: AsyncProvider + 'provider,
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_provider_async(provider);
        async_builder
    }

    /// Builds the builder.
    ///
    /// This is function that actually fetches data from all the sources and deserializes them.
    ///```rust
    ///use miau::builder::ConfigurationBuilder;
    ///use miau::source::FileSource;
    ///use miau::provider::EnvironmentProvider;
    ///use miau::format;
    ///use miau::configuration::Configuration;
    ///
    ///let mut builder = ConfigurationBuilder::default();
    ///
    ///builder.add_provider(EnvironmentProvider::default());
    ///builder.add(FileSource::from_path("./a/path/to/file.json"), format::json());
    ///
    ///let configuration : Configuration = match builder.build() {
    ///     Ok(cfg) => cfg,    
    ///     Err(e) => return
    ///};
    ///```
    pub fn build(&mut self) -> Result<Configuration, ConfigurationError> {
        let mut result = Configuration::default();

        for provider in self.sources.iter_mut() {
            let roots = provider.collect()?;
            for configuration in roots.roots {
                result.roots.push(configuration);
            }
        }

        Ok(result)
    }
}

/// Configuration builder capable of using both synchronous and asynchronous sources.
///
/// This power comes at a price - it requires executor.
/// Therefore it can only be invoked inside runtime.
///
/// Owns all sources passed to it and is capable of creating Configuration object.
///
/// Since it handles both synchronous and asynchronous sources it is possible to create
/// async builder with only synchronous sources. It is discouraged as in such case execution
/// is the same as in case of synchronous builder, but requires runtime.
pub struct AsyncConfigurationBuilder<'provider> {
    sources: Vec<SourceType<'provider>>,
}

impl<'provider> Default for AsyncConfigurationBuilder<'provider> {
    fn default() -> Self {
        AsyncConfigurationBuilder::new()
    }
}

enum SourceType<'provider> {
    Synchronous(Box<dyn Provider + 'provider>),
    Asynchronous(Box<dyn AsyncProvider + 'provider>),
}

impl<'provider> AsyncConfigurationBuilder<'provider> {
    /// Creates new builder.
    ///
    /// This function is used in Default trait implementation.
    ///```rust
    ///use miau::builder::AsyncConfigurationBuilder;
    ///
    ///let builder = AsyncConfigurationBuilder::new();
    ///```
    pub fn new() -> Self {
        AsyncConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    /// Creates asynchronous builder from synchronous one, consuming it.
    ///
    /// It should not be used directly.
    /// Instead either use async builder from the start or use one of methods of synchronous builder that convert it for you.
    ///
    /// Exposed as public API to serve strangest needs.
    pub fn from_synchronous_builder(
        mut builder: ConfigurationBuilder<'provider>,
    ) -> AsyncConfigurationBuilder<'provider> {
        AsyncConfigurationBuilder {
            sources: builder
                .sources
                .drain(..)
                .map(|s| SourceType::Synchronous(s))
                .collect(),
        }
    }

    /// Adds new synchronous source and format to builder.
    ///
    /// Similar to [`add`](ConfigurationBuilder::add()) on synchronous builder.
    pub fn add<S, D>(&mut self, source: S, format: D) -> &mut AsyncConfigurationBuilder<'provider>
    where
        S: Source + 'provider,
        D: Format + 'provider,
    {
        self.add_provider(ProviderStruct::synchronous(source, format))
    }

    /// Adds new synchronous provider to builder.
    ///
    /// Similar to [`add_provider`](ConfigurationBuilder::add_provider()) on synchronous builder.
    pub fn add_provider<P>(&mut self, provider: P) -> &mut AsyncConfigurationBuilder<'provider>
    where
        P: Provider + 'provider,
    {
        self.sources
            .push(SourceType::Synchronous(Box::new(provider)));
        self
    }

    /// Adds new asynchronous source and format to builder.
    ///
    /// Similar to [`add_async`](ConfigurationBuilder::add_async()) on synchronous builder.
    /// Unlike it, however, it is not consuming the builder.
    pub fn add_async<S, D>(
        &mut self,
        source: S,
        format: D,
    ) -> &mut AsyncConfigurationBuilder<'provider>
    where
        S: AsyncSource + Send + Sync + 'provider,
        D: Format + Send + Sync + 'provider,
    {
        self.add_provider_async(ProviderStruct::asynchronous(source, format))
    }

    /// Adds new asynchronous provider to builder.
    ///
    /// Similar to [`add_provider_async`](ConfigurationBuilder::add_provider_async()) on synchronous builder.
    /// Unlike it, however, it is not consuming the builder.
    pub fn add_provider_async<P>(
        &mut self,
        provider: P,
    ) -> &mut AsyncConfigurationBuilder<'provider>
    where
        P: AsyncProvider + 'provider,
    {
        self.sources
            .push(SourceType::Asynchronous(Box::new(provider)));
        self
    }

    /// Builds the builder.
    ///
    /// This is function that actually fetches data from all the sources and deserializes them.
    ///
    /// Since it is asynchronous, it requires runtime to be present.
    pub async fn build(&mut self) -> Result<Configuration, ConfigurationError> {
        let mut result = Configuration::default();

        for provider in self.sources.iter_mut() {
            let configuration = match provider {
                SourceType::Synchronous(provider) => provider.collect()?,
                SourceType::Asynchronous(provider) => provider.collect().await?,
            };
            for root in configuration.roots {
                result.roots.push(root);
            }
        }

        Ok(result)
    }
}
