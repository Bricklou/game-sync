use actix_web::{get, HttpResponse, Responder};

#[derive(serde::Serialize)]
struct Status {
    name: &'static str,
    version: &'static str,
    status: &'static str,
}

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        status: "UP",
    })
}
