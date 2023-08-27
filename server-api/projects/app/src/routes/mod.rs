use crate::{controllers, data::AppData};
use actix_web::web;

mod admin;
mod api;

pub fn setup_routes(cfg: &mut web::ServiceConfig, app_data: &AppData) {
    cfg.service(controllers::base::status);

    cfg.configure(|cfg| api::register_route(cfg, app_data));
    cfg.configure(|cfg| admin::register_route(cfg, app_data));
}
