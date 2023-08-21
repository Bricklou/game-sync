use actix_web::{delete, get, post, web::Data, Responder};

use crate::{
    core::{config::AppConfig, database::DbPool, types::ValidatedJson},
    models::user::UserLoginRequest,
};

#[post("")]
pub async fn login(
    input: ValidatedJson<UserLoginRequest>,
    config: Data<AppConfig>,
    db: Data<DbPool>,
) -> impl Responder {
    "Login"
}

#[get("")]
pub async fn me() -> impl Responder {
    "Refresh"
}

#[delete("")]
pub async fn logout() -> impl Responder {
    "Logout"
}
