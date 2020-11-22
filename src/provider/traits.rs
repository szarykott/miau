use crate::{
    configuration::{Configuration, ConfigurationInfo},
    error::ConfigurationError,
    format::Format,
    source::{AsyncSource, Source},
};
use async_trait::async_trait;

/// Configuration provider where distintion between source and format does not make sense or does little sense.
/// Examples might be other configurations already in memory, environment variables or others.
pub trait Provider {
    fn collect(&self) -> Result<Configuration, ConfigurationError>;
    fn describe(&self) -> ConfigurationInfo;
}

/// Async provider where distinction between source and format does not make sense or makes little sense.
#[async_trait]
pub trait AsyncProvider: Send + Sync {
    async fn collect(&self) -> Result<Configuration, ConfigurationError>;
    fn describe(&self) -> ConfigurationInfo;
}

/// Combines source and transformer into single provider.
pub struct ProviderStruct<S, T> {
    source: S,
    format: T,
}

impl<S: Source, T: Format> ProviderStruct<S, T> {
    pub fn synchronous(s: S, t: T) -> Self {
        ProviderStruct {
            source: s,
            format: t,
        }
    }
}

impl<S, T> ProviderStruct<S, T>
where
    S: AsyncSource + Send + Sync,
    T: Format + Send + Sync,
{
    pub fn asynchronous(s: S, t: T) -> Self {
        ProviderStruct {
            source: s,
            format: t,
        }
    }
}

impl<S, T> Provider for ProviderStruct<S, T>
where
    S: Source,
    T: Format,
{
    fn collect(&self) -> Result<Configuration, ConfigurationError> {
        let node = self.format.transform(self.source.collect()?)?;
        Ok(Configuration::new_singular(self.describe(), node))
    }

    fn describe(&self) -> ConfigurationInfo {
        ConfigurationInfo::new(self.source.describe(), self.format.describe())
    }
}

#[async_trait]
impl<S, T> AsyncProvider for ProviderStruct<S, T>
where
    S: AsyncSource + Send + Sync,
    T: Format + Send + Sync,
{
    async fn collect(&self) -> Result<Configuration, ConfigurationError> {
        let node = self.format.transform(self.source.collect().await?)?;
        Ok(Configuration::new_singular(self.describe(), node))
    }

    fn describe(&self) -> ConfigurationInfo {
        ConfigurationInfo::new(self.source.describe(), self.format.describe())
    }
}
