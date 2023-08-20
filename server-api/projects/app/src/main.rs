mod app;

mod controllers;
mod core;
mod entities;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    if let Err(e) = app::run().await {
        log::error!("{}", e);
    }

    Ok(())
}
