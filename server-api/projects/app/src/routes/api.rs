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
        .service(
            web::resource("games")
                .route(web::get().to(api_ctrl::games::get_games))
                .route(web::post().to(api_ctrl::games::create_game))
                .wrap(Auth),
        )
        .service(
            web::resource("games/{id}")
                .route(web::get().to(api_ctrl::games::get_game))
                .route(web::put().to(api_ctrl::games::update_game))
                .wrap(Auth),
        )
        .wrap(session_middleware);

    cfg.service(scope);
}
