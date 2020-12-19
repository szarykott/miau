use async_trait::async_trait;
use miau::{
    builder::AsyncConfigurationBuilder, configuration::ConfigurationRead,
    error::ConfigurationError, format, format::Json5, provider::EnvironmentProvider,
    source::AsyncSource,
};
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncReadExt};

// --------------------- Let's implement async file source first ----- //

pub struct AsyncFileSource {
    path: PathBuf,
}

impl AsyncFileSource {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Self {
        AsyncFileSource {
            path: path.as_ref().to_path_buf(),
        }
    }
}

// Notice [async trait here]! It is required to implement the trait.
#[async_trait]
impl AsyncSource for AsyncFileSource {
    async fn collect(&self) -> Result<Vec<u8>, ConfigurationError> {
        let mut buffer = Vec::new();

        let mut f = File::open(&self.path)
            .await
            .map_err(|e| -> ConfigurationError { e.into() })
            .map_err(|e| {
                e.enrich_with_context(format!("Failed to open file : {}", self.path.display()))
            })?;

        f.read_to_end(&mut buffer)
            .await
            .map_err(|e| -> ConfigurationError { e.into() })
            .map_err(|e| {
                e.enrich_with_context(format!("Failed to read file : {}", self.path.display()))
            })?;

        Ok(buffer)
    }

    fn describe(&self) -> String {
        std::fs::canonicalize(&self.path)
            .unwrap_or_else(|_| self.path.clone())
            .display()
            .to_string()
    }
}

// ---------------------- Actual example begins -------------------------------- //

// In real applications you should be able to use [tokio::main] or similar from other runtimes
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    runtime.block_on(async { async_main().await });
}

async fn async_main() {
    // it is also possible to start with synchronous builder and convert it to asynchronous one
    let mut builder = AsyncConfigurationBuilder::default();

    let result = builder
        // synchronous sources can be added to asynchronous builder
        .add_provider(EnvironmentProvider::default())
        // adding first async source
        .add_async(
            AsyncFileSource::from_path("./files/config.json"), // specify source first
            format::json(), // predefined formats can be specified with format::* helper methods
        )
        // adding second async source
        .add_async(
            AsyncFileSource::from_path("./files/config.json5"),
            Json5::default(), // structs implementing `Format` trait can also be used directly
        )
        .build()
        .await; // only now all values will be fetched

    let configuration = match result {
        Ok(configuration) => configuration,
        Err(e) => panic!(
            "Please make sure you run `cargo run` from examples folder! {}",
            e.pretty_display()
        ),
    };

    // more elaborate example about retrieving values can be found in `basic` example

    let from_json: Option<i32> = configuration.get("map:value"); // index into maps by using ':' between keys
    assert_eq!(Some(1), from_json);

    let from_json5: Option<bool> = configuration.get("map:boolean");
    assert_eq!(Some(false), from_json5); // notice json5 overwrites this value

    let from_array: Option<i32> = configuration.get("array:[1]"); // use [x] to mark you are indexing into array
    assert_eq!(Some(2), from_array);
}
