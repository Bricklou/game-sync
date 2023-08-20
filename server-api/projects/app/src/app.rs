use crate::{
    core::{config::AppConfig, database, errors::AppError, setup},
    routes,
};
use actix_web::{
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};

pub async fn run() -> Result<(), AppError> {
    let config = AppConfig::from_env()?;

    let pool = database::init_pool(&config.database.url).await?;

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(setup::server_setup)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
    .map_err(AppError::from)
}
