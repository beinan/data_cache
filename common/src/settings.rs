use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub master: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let s = Config::builder()
            .add_source(File::with_name("alluxio_config"))
            .build()?;
        // println!("load settings: {:?}", s);
        s.try_deserialize()
    }
}
