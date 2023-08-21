mod app;

mod controllers;
mod core;
mod entities;
mod helpers;
mod models;
mod repositories;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    if let Err(e) = app::run().await {
        log::error!("{}", e);
    }

    Ok(())
}
