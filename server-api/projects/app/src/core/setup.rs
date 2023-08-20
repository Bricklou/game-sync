use actix_web::web::ServiceConfig;

use crate::routes;

pub fn server_setup(cfg: &mut ServiceConfig) {
    // Register routes
    routes::setup_routes(cfg);
}
