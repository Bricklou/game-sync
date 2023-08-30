use actix_multi_session::storage::RedisSessionStore;
use tera::Tera;

use crate::{
    core::{config::AppConfig, database, errors::AppError},
    data::AppData,
};

pub async fn init() -> Result<AppData, AppError> {
    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize template engine (Tera)
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    // Initialize database
    let pool = database::init_pool(&config.database.url).await?;
    database::seed_database(&pool).await?;

    // Initialize session middleware
    let redis_store = RedisSessionStore::new(&config.redis.url)
        .await
        .map_err(AppError::from)?;

    // Create secret key
    let secret_key = config.server.secret_key.clone();

    // Initialize application data
    let app_data = AppData {
        tera,
        db: pool,
        session_store: redis_store,
        config,
        secret_key,
    };

    Ok(app_data)
}
