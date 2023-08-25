use actix_web::web;
use actix_web_lab::web::spa;

use crate::{controllers, middlewares};

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::base::status)
        .service(
            web::scope("api")
                // Auth routes
                .service(
                    web::scope("auth")
                        .service(controllers::auth::login)
                        .service(
                            web::scope("")
                                .wrap(middlewares::auth::Auth)
                                .service(controllers::auth::me)
                                .service(controllers::auth::refresh)
                                .service(controllers::auth::logout),
                        ),
                ),
        )
        .service(
            // Admin routes
            web::scope("admin"),
        );
}
