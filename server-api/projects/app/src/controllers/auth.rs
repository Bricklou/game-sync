use actix_web::{
    delete, get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    core::{config::SecretKey, database::DbPool, errors::AppError, types::ValidatedJson},
    entities::user::Model as UserModel,
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

    // TODO: return session token through cookies or OAT !!! (cookie for web and OAT for desktop)

    //let tokens = jwt::generate_tokens_response(&user, &secret_key)?;
    return Ok(HttpResponse::Ok().json(tokens));
}

#[get("")]
pub async fn me(user: web::ReqData<UserModel>) -> impl Responder {
    // Return the user data
    HttpResponse::Ok().json(user.into_inner())
}

#[delete("")]
pub async fn logout() -> impl Responder {
    "Logout"
}

#[post("/refresh")]
pub async fn refresh() -> impl Responder {
    "Refresh"
}
