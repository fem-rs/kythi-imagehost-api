use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("Config"))
            .build()?;

        builder.try_deserialize()
    }
}
