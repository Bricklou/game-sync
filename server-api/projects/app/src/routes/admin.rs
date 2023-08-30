use crate::data::AppData;
use crate::middlewares::auth::Auth;
use crate::{controllers::admin as admin_ctrl, middlewares::guest::Guest};
use actix_multi_session::provider::CookieTokenProvider;
use actix_multi_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::web::{self, ServiceConfig};
use time::Duration;

pub fn register_route(cfg: &mut ServiceConfig, app_data: &AppData) {
    let store = app_data.session_store.clone();
    let key = app_data.secret_key.clone();

    let session_provider = CookieTokenProvider::builder(Key::derive_from(key.0.as_bytes()))
        .cookie_name("game-sync-session".to_string())
        .cookie_http_only(true)
        .cookie_secure(true)
        .session_ttl(Duration::weeks(1))
        .build();

    let session_middleware = SessionMiddleware::builder(store, session_provider).build();

    let scope = web::scope("admin")
        .service(admin_ctrl::admin::index)
        .service(
            web::scope("auth")
                .service(
                    web::resource("")
                        .route(web::get().to(admin_ctrl::auth::me).wrap(Auth))
                        .route(web::delete().to(admin_ctrl::auth::logout).wrap(Auth))
                        .route(web::post().to(admin_ctrl::auth::login).wrap(Guest)),
                )
                .route(
                    "register",
                    web::route().to(admin_ctrl::auth::register).wrap(Guest),
                ),
        )
        .wrap(session_middleware);

    cfg.service(scope);
}
