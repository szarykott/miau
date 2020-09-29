use crate::{
    configuration::Configuration,
    de::ConfigurationDeserializer,
    error::ConfigurationError,
    source::{AsyncSource, Source},
};

pub struct ConfigurationBuilder<'a> {
    sources: Vec<(
        Box<dyn Source + 'a>,
        Box<dyn ConfigurationDeserializer + 'a>,
    )>,
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
        D: ConfigurationDeserializer + 'a,
    {
        self.sources.push((Box::new(source), Box::new(de)));
        self
    }

    pub fn add_async<S, D>(self, source: S, de: D) -> AsyncConfigurationBuilder<'a>
    where
        S: AsyncSource + 'a,
        D: ConfigurationDeserializer + 'a,
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_async(source, de);
        async_builder
    }

    pub fn build(&mut self) -> Result<Configuration, ConfigurationError> {
        if self.sources.len() > 1 {
            let (source, de) = self.sources.remove(0);
            let input = source.collect()?;
            let mut result = de.deserialize(input)?;

            for (source, de) in self.sources.iter() {
                let input = source.collect()?;
                let configuration = de.deserialize(input)?;
                result = Configuration::merge(result, configuration).unwrap(); // TODO: Fix it
            }

            Ok(result)
        } else if self.sources.len() == 1 {
            let (source, de) = self.sources.remove(0);
            let input = source.collect()?;
            Ok(de.deserialize(input)?)
        } else {
            panic!("Empty builder") //TODO: Do not panic!
        }
    }
}

pub struct AsyncConfigurationBuilder<'a> {
    sources: Vec<(SourceType<'a>, Box<dyn ConfigurationDeserializer + 'a>)>,
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
        D: ConfigurationDeserializer + 'a,
    {
        self.sources
            .push((SourceType::Synchronous(Box::new(source)), Box::new(de)));
    }

    pub fn add_async<S, D>(&mut self, source: S, de: D)
    where
        S: AsyncSource + 'a,
        D: ConfigurationDeserializer + 'a,
    {
        self.sources
            .push((SourceType::Asynchronous(Box::new(source)), Box::new(de)));
    }
}

enum SourceType<'a> {
    Synchronous(Box<dyn Source + 'a>),
    Asynchronous(Box<dyn AsyncSource + 'a>),
}
