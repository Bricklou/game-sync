use crate::core::{config::AppConfig, database, errors::AppError, setup};
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use time::Duration;
use tracing::info;

pub async fn run() -> Result<(), AppError> {
    // Load configuration
    let config = AppConfig::from_env()?;
    let ip_addr = IpAddr::from_str(config.server.host.as_str())?;
    let addrs = SocketAddr::new(ip_addr, config.server.port);

    // Initialize database
    let pool = database::init_pool(&config.database.url).await?;

    // Initialize session middleware
    let store = RedisSessionStore::new(&config.redis.url)
        .await
        .map_err(AppError::from)?;
    let key = config.server.secret_key.clone();

    // Start server
    info!("Starting server at http://{}:{}", addrs.ip(), addrs.port());

    HttpServer::new(move || {
        let config = config.clone();

        let session_middleware =
            SessionMiddleware::builder(store.clone(), Key::derive_from(key.0.as_bytes()))
                .cookie_http_only(true)
                .cookie_secure(true)
                .cookie_name("game-sync-session".to_string())
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::weeks(1)))
                .build();

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.server.secret_key.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(session_middleware)
            .configure(setup::server_setup)
    })
    .bind(addrs)?
    .run()
    .await
    .map_err(AppError::from)
}
