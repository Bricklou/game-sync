use config::ConfigError;

#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,

    pub secret_key: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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
