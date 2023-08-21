use crate::{
    core::{config::AppConfig, database, errors::AppError, setup},
    routes,
};
use actix_web::{
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use tracing::info;

pub async fn run() -> Result<(), AppError> {
    // Load configuration
    let config = AppConfig::from_env()?;

    // Initialize database
    let pool = database::init_pool(&config.database.url).await?;
    // Run database migrations
    database::seed_database(&pool).await?;

    // Start server
    info!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.server.secret_key.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(setup::server_setup)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
    .map_err(AppError::from)
}
