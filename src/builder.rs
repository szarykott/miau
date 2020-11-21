use crate::{
    configuration::Configuration,
    error::ConfigurationError,
    format::Format,
    provider::{AsyncProvider, Provider, ProviderStruct},
    source::{AsyncSource, Source},
};
use std::default::Default;

pub struct ConfigurationBuilder<'provider> {
    sources: Vec<Box<dyn Provider + 'provider>>,
}

impl<'provider> Default for ConfigurationBuilder<'provider> {
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

/// Holds intermediate configuration sources in order of adding them.
impl<'provider> ConfigurationBuilder<'provider> {
    fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    /// Core function to add new configurations to builder.
    pub fn add<S, D>(&mut self, source: S, format: D) -> &mut ConfigurationBuilder<'provider>
    where
        S: Source + 'provider,
        D: Format + 'provider,
    {
        self.add_provider(ProviderStruct::synchronous(source, format));
        self
    }

    pub fn add_provider<P>(&mut self, provider: P) -> &mut ConfigurationBuilder<'provider>
    where
        P: Provider + 'provider,
    {
        self.sources.push(Box::new(provider));
        self
    }

    pub fn add_async<S, D>(self, source: S, format: D) -> AsyncConfigurationBuilder<'provider>
    where
        S: AsyncSource + Send + Sync + 'provider,
        D: Format + Send + Sync + 'provider,
    {
        self.add_provider_async(ProviderStruct::asynchronous(source, format))
    }

    pub fn add_provider_async<P>(self, provider: P) -> AsyncConfigurationBuilder<'provider>
    where
        P: AsyncProvider + 'provider,
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_provider_async(provider);
        async_builder
    }

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

// TODO: test async builder
impl<'provider> AsyncConfigurationBuilder<'provider> {
    pub fn new() -> Self {
        AsyncConfigurationBuilder {
            sources: Vec::new(),
        }
    }

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

    pub fn add<S, D>(&mut self, source: S, format: D) -> &mut AsyncConfigurationBuilder<'provider>
    where
        S: Source + 'provider,
        D: Format + 'provider,
    {
        self.add_provider(ProviderStruct::synchronous(source, format))
    }

    pub fn add_provider<P>(&mut self, provider: P) -> &mut AsyncConfigurationBuilder<'provider>
    where
        P: Provider + 'provider,
    {
        self.sources
            .push(SourceType::Synchronous(Box::new(provider)));
        self
    }

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
