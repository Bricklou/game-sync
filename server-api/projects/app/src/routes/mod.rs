use actix_web::web;

use crate::controllers;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Base routes
            .service(controllers::base::status)
            // Auth routes
            .service(
                web::scope("/auth")
                    .service(controllers::auth::login)
                    .service(controllers::auth::me)
                    .service(controllers::auth::logout),
            ),
    );
}
