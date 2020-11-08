use crate::{
    configuration::Configuration,
    error::ConfigurationError,
    format::Transformer,
    source::{AsyncSource, Source},
};
use async_trait::async_trait;

/// Configuration provider for which distintion between source and format does not make sense or does little sense.
/// Examples might be other configurations already in memory, environment variables or others.

pub trait Provider {
    fn collect(&self) -> Result<Configuration, ConfigurationError>;
}

/// Async provider for which distintion between source and format does not make sense or makes little sense.
#[async_trait]
pub trait AsyncProvider {
    async fn collect(&self) -> Result<Configuration, ConfigurationError>;
}

/// Combines source and transformer into single provider.
pub struct ProviderStruct<S, T> {
    source: S,
    transformer: T,
}

impl<S: Source, T: Transformer> ProviderStruct<S, T> {
    pub fn synchronous(s: S, t: T) -> Self {
        ProviderStruct {
            source: s,
            transformer: t,
        }
    }
}

impl<S, T> ProviderStruct<S, T>
where
    S: AsyncSource + Send + Sync,
    T: Transformer + Send + Sync,
{
    pub fn asynchronous(s: S, t: T) -> Self {
        ProviderStruct {
            source: s,
            transformer: t,
        }
    }
}

impl<S: Source, T: Transformer> Provider for ProviderStruct<S, T> {
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        self.transformer.transform(self.source.collect()?)
    }
}

#[async_trait]
impl<S, T> AsyncProvider for ProviderStruct<S, T>
where
    S: AsyncSource + Send + Sync,
    T: Transformer + Send + Sync,
{
    async fn collect(&self) -> Result<Configuration, ConfigurationError> {
        self.transformer.transform(self.source.collect().await?)
    }
}
