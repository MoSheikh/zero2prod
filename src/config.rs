use secrecy::*;
use serde::Deserialize;

const CONF_DIR: &str = "conf";

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
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app: AppSettings,
}

impl Settings {
    pub fn get() -> Result<Settings, config::ConfigError> {
        let base_dir = std::env::current_dir().expect("Failed to determine the current directory");
        let conf_dir = base_dir.join(CONF_DIR);

        let env: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| {
                tracing::info!("`APP_ENV` is not set - defaulting to \"local\"");
                "local".into()
            })
            .try_into()
            .expect("Failed to parse `APP_ENV`");
        let env_filename = format!("{}.yaml", env.as_str());
        config::Config::builder()
            .add_source(config::File::from(conf_dir.join("base.yaml")))
            .add_source(config::File::from(conf_dir.join(env_filename)))
            .build()?
            .try_deserialize::<Self>()
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

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
		 Use either `local` or `production`.",
                other
            )),
        }
    }
}
