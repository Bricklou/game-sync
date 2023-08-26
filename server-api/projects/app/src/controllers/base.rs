use actix_web::{get, web, HttpResponse, Responder};

use crate::{core::database::DbPool, repositories::app};

#[derive(serde::Serialize)]
struct Status {
    name: &'static str,
    version: &'static str,
    status: &'static str,
    configured: bool,
}

#[get("/")]
pub async fn status(db: web::Data<DbPool>) -> impl Responder {
    let is_conn_ok = db.ping().await.is_ok();

    if !is_conn_ok {
        return HttpResponse::ServiceUnavailable().json(Status {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            status: "DOWN",
            configured: false,
        });
    }

    let is_configured = app::is_configured(&db).await.unwrap_or(false);

    return HttpResponse::Ok().json(Status {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        status: "UP",
        configured: is_configured,
    });
}
