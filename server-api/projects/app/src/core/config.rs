use config::ConfigError;

#[derive(serde::Deserialize, Clone)]
pub struct SecretKey(pub String);

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    pub secret_key: SecretKey,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct StorageConfig {
    pub name: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    #[serde(default = "default_use_ssl")]
    pub use_ssl: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub storage: StorageConfig,

    #[serde(default = "default_enable_registration")]
    pub enable_registration: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::with_prefix("GAME_SYNC").separator("_"))
            .add_source(config::File::with_name("config").required(false))
            .build()?;

        let cfg: AppConfig = cfg.try_deserialize()?;
        Ok(cfg)
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8431
}

fn default_enable_registration() -> bool {
    false
}

fn default_use_ssl() -> bool {
    true
}
