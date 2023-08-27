use actix_web::web::{self, ServiceConfig};
use time::Duration;

use crate::{
    controllers::api as api_ctrl,
    data::AppData,
    helpers::opaque_token::middleware::SessionOatMiddleware,
    middlewares::{self, guest::Guest},
};

pub fn register_route(cfg: &mut ServiceConfig, app_data: &AppData) {
    let store = app_data.session_store.clone();

    let session_middleware = SessionOatMiddleware::builder(store)
        .session_ttl(Duration::weeks(1))
        .build();

    let scope = web::scope("api")
        // Auth routes
        .service(
            web::scope("auth")
                .service(web::scope("").service(api_ctrl::auth::login).wrap(Guest))
                .service(
                    web::scope("")
                        .wrap(middlewares::auth::Auth)
                        .service(api_ctrl::auth::me)
                        .service(api_ctrl::auth::logout)
                        .wrap(session_middleware),
                ),
        );

    cfg.service(scope);
}
