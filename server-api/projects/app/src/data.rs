use std::net::SocketAddr;

use actix_multi_session::storage::RedisSessionStore;
use tera::Tera;

use crate::core::{
    config::{AppConfig, SecretKey},
    database::DbPool,
    errors::AppResult,
    s3::S3Client,
};

#[derive(Clone)]
pub struct AppData {
    pub tera: Tera,
    pub db: DbPool,
    pub session_store: RedisSessionStore,
    pub config: AppConfig,
    pub secret_key: SecretKey,
    pub s3: S3Client,
}

impl AppData {
    pub fn get_server_address(&self) -> AppResult<SocketAddr> {
        let ip_addr = self.config.server.host.parse()?;
        Ok(SocketAddr::new(ip_addr, self.config.server.port))
    }
}
