use config::{Config, ConfigError};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mode: String,
    pub tldr_settings: TldrSettings,
    pub git_settings: GitSettings,
    pub storage_settings: StorageSettings,
}

#[derive(Debug, Deserialize)]
pub struct TldrSettings {
    pub mark_email_as_read: bool
}
#[derive(Debug, Deserialize)]
pub struct GitSettings {
    pub enable: bool,
    pub branch: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageSettings {
    pub repository_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let config_builder = Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP"));

        config_builder.build()?.try_deserialize()
    }
}
