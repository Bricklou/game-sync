use crate::{
    bootstrap,
    core::{errors::AppError, setup},
};
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use tracing::info;

pub async fn run() -> Result<(), AppError> {
    // Bootstrap application
    let app_data = bootstrap::init().await?;

    // Load configuration
    let addrs = app_data.get_server_address()?;

    // Start server
    info!("Starting server at http://{}:{}", addrs.ip(), addrs.port());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(|cfg| setup::server_setup(cfg, app_data.clone()))
    })
    .bind(addrs)?
    .run()
    .await
    .map_err(AppError::from)
}
