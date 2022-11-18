use std::path::PathBuf;

use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub app: AppConfig,
    pub log: LogConfig,
    pub server: ServerConfig,
    pub database_url: String,
    pub jwt_pub_key: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub env: Environment,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Prod,
    Dev,
    Test,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub format: LogFormat,
    pub level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Full,
    JSON,
    Pretty,
    Compact,
}

pub fn get_config() -> Configuration {
    config::Config::builder()
        .add_source(
            glob("conf/*.toml")
                .unwrap()
                .map(|path| config::File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .add_source(config::Environment::default())
        .build()
        .expect("failed to load configuration")
        .try_deserialize::<Configuration>()
        .expect("Failed to deserialize app configuration")
}
