use crate::{
    configuration::Configuration,
    error::ConfigurationError,
    format::Transformer,
    source::{AsyncSource, DummySource, Source},
};

use std::default::Default;

pub struct ConfigurationBuilder<'a> {
    sources: Vec<(Box<dyn Source + 'a>, Box<dyn Transformer + 'a>)>,
}

impl<'a> Default for ConfigurationBuilder<'a> {
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

/// Holds intermediate configuration sources in order of adding them.
impl<'a> ConfigurationBuilder<'a> {
    pub fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn add<S, D>(&mut self, source: S, de: D) -> &mut ConfigurationBuilder<'a>
    where
        S: Source + 'a,
        D: Transformer + 'a,
    {
        self.sources.push((Box::new(source), Box::new(de)));
        self
    }

    pub fn add_existing<D>(&mut self, de: D) -> &mut ConfigurationBuilder<'a>
    where
        D: Transformer + 'a,
    {
        self.sources.push((Box::new(DummySource), Box::new(de)));
        self
    }

    pub fn add_async<S, D>(self, source: S, de: D) -> AsyncConfigurationBuilder<'a>
    where
        S: AsyncSource + 'a,
        D: Transformer + 'a,
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_async(source, de);
        async_builder
    }

    pub fn build(&mut self) -> Result<Configuration, ConfigurationError> {
        let mut result = Configuration::default();

        for (source, de) in self.sources.iter_mut() {
            let roots = de.transform(source.collect()?)?;
            for configuration in roots.roots {
                result.roots.push(configuration);
            }
        }

        Ok(result)
    }
}

pub struct AsyncConfigurationBuilder<'a> {
    sources: Vec<(SourceType<'a>, Box<dyn Transformer + 'a>)>,
}

impl<'a> AsyncConfigurationBuilder<'a> {
    pub fn new() -> Self {
        AsyncConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn from_synchronous_builder(
        mut builder: ConfigurationBuilder<'a>,
    ) -> AsyncConfigurationBuilder<'a> {
        AsyncConfigurationBuilder {
            sources: builder
                .sources
                .drain(..)
                .map(|s| (SourceType::Synchronous(s.0), s.1))
                .collect(),
        }
    }

    pub fn add<S, D>(&mut self, source: S, de: D)
    where
        S: Source + 'a,
        D: Transformer + 'a,
    {
        self.sources
            .push((SourceType::Synchronous(Box::new(source)), Box::new(de)));
    }

    pub fn add_async<S, D>(&mut self, source: S, de: D)
    where
        S: AsyncSource + 'a,
        D: Transformer + 'a,
    {
        self.sources
            .push((SourceType::Asynchronous(Box::new(source)), Box::new(de)));
    }
}

enum SourceType<'a> {
    Synchronous(Box<dyn Source + 'a>),
    Asynchronous(Box<dyn AsyncSource + 'a>),
}
