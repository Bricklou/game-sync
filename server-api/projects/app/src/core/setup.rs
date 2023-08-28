use actix_web::web::{Data, JsonConfig, ServiceConfig};

use crate::{data::AppData, routes};

use super::errors;

pub fn server_setup(cfg: &mut ServiceConfig, app_data: AppData) {
    // Register data
    cfg.app_data(Data::new(app_data.clone()));

    // Register error handlers
    let json_config = JsonConfig::default().error_handler(errors::json_error_handler);
    cfg.app_data(json_config);

    let validated_json_config = actix_web_validator::JsonConfig::default()
        .error_handler(errors::validated_json_error_handler);
    cfg.app_data(validated_json_config);

    // Register routes
    cfg.configure(|cfg| routes::setup_routes(cfg, &app_data));
}
