use actix_web::{get, web, HttpResponse, Responder};

use crate::{data::AppData, repositories::app};

#[derive(serde::Serialize)]
struct Status {
    name: &'static str,
    version: &'static str,
    status: &'static str,
    configured: bool,
}

#[get("/")]
pub async fn status(data: web::Data<AppData>) -> impl Responder {
    let is_conn_ok = data.db.ping().await.is_ok();

    let mut status = Status {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        status: "UP",
        configured: false,
    };

    if !is_conn_ok {
        return HttpResponse::ServiceUnavailable().json(status);
    }

    let is_configured = app::is_configured(&data.db).await.unwrap_or(false);

    status.configured = is_configured;

    return HttpResponse::Ok().json(status);
}
