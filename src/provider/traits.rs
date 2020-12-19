use crate::{
    configuration::{Configuration, ConfigurationInfo},
    error::ConfigurationError,
    format::Format,
    source::{AsyncSource, Source},
};
use async_trait::async_trait;

/// Represents configuration source and its associated format.
///
/// Can be as an aggregator of the two or by itself to represent source in which distinction between source and format is blurry.
pub trait Provider {
    /// Collects given source into `Configuration`.
    fn collect(&self) -> Result<Configuration, ConfigurationError>;
    /// Describes this provider.
    fn describe(&self) -> ConfigurationInfo;
}

/// Represents asynchronous configuration source and its associated format.
///
/// Can be as an aggregator of the two or by itself to represent source in which distinction between source and format is blurry.
#[async_trait]
pub trait AsyncProvider: Send + Sync {
    /// Collects given source into `Configuration`.
    /// It uses [`async_trait`](async_trait::async_trait) which is required to implement `AsyncSource`.
    async fn collect(&self) -> Result<Configuration, ConfigurationError>;
    /// Describes this provider.
    fn describe(&self) -> ConfigurationInfo;
}

/// Combines source and format into single provider.
pub struct ProviderStruct<S, T> {
    source: S,
    format: T,
}

impl<S: Source, T: Format> ProviderStruct<S, T> {
    /// Constructs new synchronous source provider.
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
    /// Constructs new asynchronous source provider.
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
