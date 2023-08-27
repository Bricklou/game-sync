use crate::data::AppData;
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_web::{
    cookie::Key,
    web::{self, ServiceConfig},
    HttpResponse,
};
use time::Duration;

pub fn register_route(cfg: &mut ServiceConfig, app_data: &AppData) {
    let store = app_data.session_store.clone();
    let key = app_data.secret_key.clone();

    let session_middleware = SessionMiddleware::builder(store, Key::derive_from(key.0.as_bytes()))
        .cookie_http_only(true)
        .cookie_secure(true)
        .cookie_name("game-sync-session".to_string())
        .session_lifecycle(PersistentSession::default().session_ttl(Duration::weeks(1)))
        .build();

    let scope = web::scope("admin")
        .route("/", web::get().to(|| HttpResponse::Ok()))
        .wrap(session_middleware);

    cfg.service(scope);
}
