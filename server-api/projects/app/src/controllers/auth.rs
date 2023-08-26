use actix_session::Session;
use actix_web::{
    delete, get, HttpResponse,
    post,
    Responder, web::{Data, ReqData},
};

use crate::{
    core::{database::DbPool, errors::AppError, types::ValidatedJson},
    entities::user::Model as UserModel,
    models::user::UserLoginRequest,
    repositories,
};

#[post("")]
pub async fn login(
    input: ValidatedJson<UserLoginRequest>,
    db: Data<DbPool>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let user = repositories::user::login(&db, &input).await?;

    // store the user_id in the session
    session.insert("user_id", &user.id)?;

    //let tokens = jwt::generate_tokens_response(&user, &secret_key)?;
    return Ok(HttpResponse::Ok().json(user));
}

#[get("")]
pub async fn me(user: ReqData<UserModel>) -> impl Responder {
    // Return the user data
    HttpResponse::Ok().json(user.into_inner())
}

#[delete("")]
pub async fn logout(session: Session) -> impl Responder {
    // Remove the user_id from the session
    session.remove("user_id");

    HttpResponse::Ok().finish()
}
