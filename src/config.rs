use config::{Config, ConfigError};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct GlobalProperties {
    pub mode: String,
    pub tldr: TldrProperties,
    pub git: GitProperties,
    pub storage: StorageProperties,
    pub ai: AiProperties,
    pub notification: NotificationProperties,
}

#[derive(Debug, Deserialize)]
pub struct TldrProperties {
    pub mark_email_as_read: bool
}
#[derive(Debug, Deserialize)]
pub struct GitProperties {
    pub enable: bool,
    pub branch: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageProperties {
    pub repository_path: String,
}

#[derive(Debug, Deserialize)]
pub struct AiProperties {
    pub enable: bool,
    pub api_key: String,
    pub model: String,
    pub article_resume_system_prompt_path: String,
    pub global_resume_system_prompt_path: String,
    pub user_prompt_path: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationProperties {
    pub discord: DiscordProperties
}

#[derive(Debug, Deserialize)]
pub struct DiscordProperties {
    pub enable: bool,
    pub webhook_url: String,
}

impl GlobalProperties {
    pub fn new() -> Result<Self, ConfigError> {

        let path: PathBuf = if let Ok(cargo_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut dev_path = PathBuf::from(cargo_dir);
            dev_path.push("config");
            dev_path 
        } else {
            let mut exec_path = env::current_exe().unwrap();
            exec_path.pop();
            exec_path.push("config");
            exec_path 
        };

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let config_builder = Config::builder()
            .add_source(config::File::with_name(&format!("{}/default",path.to_str().unwrap())))
            .add_source(config::File::with_name(&format!("{}/config/{}", path.to_str().unwrap(),run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP"));
        config_builder.build()?.try_deserialize()
    }
}
