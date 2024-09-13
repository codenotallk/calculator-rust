use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CalculateConfig {
    pub port: String,
    pub topic: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct NotifierConfig {
    pub file: String,
    pub token: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ReportConfig {
    pub port: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StoreConfig {
    pub topic: String,
    pub group: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BrokerConfig {
    pub hosts: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub calculate: CalculateConfig,
    pub notifier: NotifierConfig,
    pub report: ReportConfig,
    pub store: StoreConfig,
    pub broker: BrokerConfig,
    pub database: DatabaseConfig,
}

pub fn get_config() -> Result<Config, String> {
    match Config::from_config_file("./config.toml") {
        Ok(config) => Ok(config),
        Err(err) => Err(err.to_string()),
    }
}
