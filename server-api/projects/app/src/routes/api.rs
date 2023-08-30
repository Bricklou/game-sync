use actix_multi_session::{provider::OpaqueTokenProvider, SessionMiddleware};
use actix_web::web::{self, ServiceConfig};

use crate::{
    controllers::api as api_ctrl,
    data::AppData,
    middlewares::{self, guest::Guest},
};

pub fn register_route(cfg: &mut ServiceConfig, app_data: &AppData) {
    let store = app_data.session_store.clone();

    let session_provider = OpaqueTokenProvider::new();

    let session_middleware = SessionMiddleware::builder(store, session_provider).build();

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
