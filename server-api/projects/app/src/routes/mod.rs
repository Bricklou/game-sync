use actix_web::web;

use crate::controllers;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::base::status)
        .service(
            web::scope("api")
                // Auth routes
                .service(
                    web::scope("auth")
                        .service(controllers::auth::login)
                        .service(controllers::auth::me)
                        .service(controllers::auth::logout),
                ),
        )
        .service(web::scope("admin").service(controllers::admin::index));
}
