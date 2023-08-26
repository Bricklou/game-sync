use actix_web::{web, HttpResponse};

pub fn register_route() -> actix_web::Scope {
    web::scope("admin")
        .route("/", web::get().to(|| HttpResponse::Ok()))
}
