mod app;

mod bootstrap;
mod controllers;
mod core;
mod data;
mod entities;
mod helpers;
mod middlewares;
mod models;
mod repositories;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    if let Err(e) = app::run().await {
        tracing::error!("{}", e);
    }

    Ok(())
}
