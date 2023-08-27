use actix_web::web::{Data, ServiceConfig};

use crate::{data::AppData, routes};

pub fn server_setup(cfg: &mut ServiceConfig, app_data: AppData) {
    // Register routes
    cfg.configure(|cfg| routes::setup_routes(cfg, &app_data));

    // Register data
    cfg.app_data(Data::new(app_data.clone()));
}
