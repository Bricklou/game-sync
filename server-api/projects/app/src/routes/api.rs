use actix_multi_session::{provider::OpaqueTokenProvider, SessionMiddleware};
use actix_web::web::{self, ServiceConfig};

use crate::{
    controllers::api as api_ctrl,
    data::AppData,
    middlewares::{auth::Auth, guest::Guest},
};

pub fn register_route(cfg: &mut ServiceConfig, app_data: &AppData) {
    let store = app_data.session_store.clone();

    let session_provider = OpaqueTokenProvider::new();

    let session_middleware = SessionMiddleware::builder(store, session_provider).build();

    let scope = web::scope("api")
        // Auth routes
        .service(
            web::resource("auth")
                .route(web::get().to(api_ctrl::auth::me).wrap(Auth))
                .route(web::delete().to(api_ctrl::auth::logout).wrap(Auth))
                .route(web::post().to(api_ctrl::auth::login).wrap(Guest)),
        )
        .wrap(session_middleware);

    cfg.service(scope);
}
