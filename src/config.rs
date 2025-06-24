use secrecy::*;
use serde::Deserialize;

const CONFIG_FILE: &str = "config.yaml";

#[derive(Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: SecretString,
    pub db_name: String,
    pub host: String,
    pub port: u16,
    pub pool_size: u16,
}

impl DbSettings {
    pub fn to_url(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name,
        ))
    }
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
}

impl Settings {
    pub fn get() -> Settings {
        get_settings_from_yaml(CONFIG_FILE)
    }
}

pub fn get_settings_from_yaml(file_name: &str) -> Settings {
    config::Config::builder()
        .add_source(config::File::new(file_name, config::FileFormat::Yaml))
        .build()
        .unwrap_or_else(|e| panic!("Could not parse {file_name}: {e}"))
        .try_deserialize()
        .unwrap_or_else(|e| panic!("Could not deserialize parsed settings: {e}"))
}
