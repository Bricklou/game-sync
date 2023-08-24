use actix_web::{delete, get, post, web::Data, HttpResponse, Responder};

use crate::{
    core::{
        config::{AppConfig, SecretKey},
        database::DbPool,
        errors::AppError,
        types::ValidatedJson,
    },
    helpers::jwt,
    models::user::UserLoginRequest,
    repositories,
};

#[post("")]
pub async fn login(
    input: ValidatedJson<UserLoginRequest>,
    secret_key: Data<SecretKey>,
    db: Data<DbPool>,
) -> Result<impl Responder, AppError> {
    let user = repositories::user::login(&db, &input).await?;

    let tokens = jwt::generate_tokens_response(&user, &secret_key)?;
    return Ok(HttpResponse::Ok().json(tokens));
}

#[get("")]
pub async fn me(secret_key: Data<SecretKey>, db: Data<DbPool>) -> impl Responder {
    //let user = repositories::user::get_by_id(1).await.unwrap();
    "Me"
}

#[delete("")]
pub async fn logout() -> impl Responder {
    "Logout"
}
