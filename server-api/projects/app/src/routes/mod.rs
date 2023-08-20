use actix_web::web;

use crate::controllers;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::base::status);
}
