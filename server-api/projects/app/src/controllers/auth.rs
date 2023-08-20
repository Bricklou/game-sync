use actix_web::{get, Responder};

#[get("")]
pub async fn login() -> impl Responder {
    "Login"
}
