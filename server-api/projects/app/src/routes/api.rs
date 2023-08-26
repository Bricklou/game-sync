use actix_web::web;

use crate::{controllers, middlewares};

pub fn register_route() -> actix_web::Scope {
    web::scope("api")
        // Auth routes
        .service(
            web::scope("auth")
                .service(controllers::auth::login)
                .service(
                    web::scope("")
                        .wrap(middlewares::auth::Auth)
                        .service(controllers::auth::me)
                        .service(controllers::auth::logout),
                ),
        )
}
