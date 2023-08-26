use actix_web::web;
use crate::controllers;

mod admin;
mod api;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::base::status);

    cfg.service(api::register_route())
        .service(admin::register_route());
}
