use crate::source::{AsyncSource, Source};

struct ConfigurationBuilder {
    sources: Vec<Box<dyn Source>>,
}

/// Holds intermediate configuration sources in order of adding them.
impl ConfigurationBuilder {
    pub fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn add(&mut self, source: Box<dyn Source>) -> &mut Self {
        self.sources.push(source);
        self
    }

    pub fn add_async(self, source: Box<dyn AsyncSource>) -> AsyncConfigurationBuilder {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_async(source);
        async_builder
    }
}

struct AsyncConfigurationBuilder {
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

    pub fn add(&mut self, source: Box<dyn Source>) {
        self.sources.push(SourceType::Synchronous(source));
    }

    pub fn add_async(&mut self, source: Box<dyn AsyncSource>) {
        self.sources.push(SourceType::Asynchronous(source));
    }
}

enum SourceType {
    Synchronous(Box<dyn Source>),
    Asynchronous(Box<dyn AsyncSource>),
}
