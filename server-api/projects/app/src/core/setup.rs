use actix_web::web::{Data, ServiceConfig};
use tera::Tera;

use crate::routes;

pub fn server_setup(cfg: &mut ServiceConfig) {
    // Register routes
    routes::setup_routes(cfg);

    // Register template engine (Tera)
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    cfg.app_data(Data::new(tera));
}
