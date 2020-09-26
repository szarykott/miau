use crate::source::{AsyncSource, Source};
use crate::configuration::Configuration;
use crate::error::SourceDeserializationError;

pub struct ConfigurationBuilder {
    sources: Vec<Box<dyn Source>>,
}

/// Holds intermediate configuration sources in order of adding them.
impl ConfigurationBuilder {
    pub fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn add<D>(&mut self, source: Box<dyn Source>, de : D) -> &mut Self 
    where 
        D : FnOnce(String) -> Result<Configuration, SourceDeserializationError>
    {
        self.sources.push(source);
        self
    }

    pub fn add_async<D>(self, source: Box<dyn AsyncSource>, de : D) -> AsyncConfigurationBuilder 
    where 
        D : FnOnce(String) -> Result<Configuration, SourceDeserializationError>
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_async(source, de);
        async_builder
    }
}

pub struct AsyncConfigurationBuilder {
    sources: Vec<SourceType>,
}

impl AsyncConfigurationBuilder {
    pub fn new() -> Self {
        AsyncConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn from_synchronous_builder(mut builder: ConfigurationBuilder) -> Self {
        AsyncConfigurationBuilder {
            sources: builder
                .sources
                .drain(..)
                .map(|s| SourceType::Synchronous(s))
                .collect(),
        }
    }

    pub fn add<D>(&mut self, source: Box<dyn Source>, de : D) 
    where 
        D : FnOnce(String) -> Result<Configuration, SourceDeserializationError>
    {
        self.sources.push(SourceType::Synchronous(source));
    }

    pub fn add_async<D>(&mut self, source: Box<dyn AsyncSource>, de : D) 
    where 
        D : FnOnce(String) -> Result<Configuration, SourceDeserializationError>
    {
        self.sources.push(SourceType::Asynchronous(source));
    }
}

enum SourceType {
    Synchronous(Box<dyn Source>),
    Asynchronous(Box<dyn AsyncSource>),
}
