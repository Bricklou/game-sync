use actix_multi_session::Session;
use actix_web::{
    web::{Data, ReqData},
    HttpResponse, Responder,
};

use crate::{
    core::{errors::AppError, types::ValidatedJson},
    data::AppData,
    entities::user::Model as UserModel,
    models::user::UserLoginRequest,
    repositories,
};

pub async fn login(
    input: ValidatedJson<UserLoginRequest>,
    data: Data<AppData>,
    session: Session,
) -> Result<impl Responder, AppError> {
    let user = repositories::user::login(&data.db, &input).await?;

    session.insert("user_id", user.id)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn me(user: ReqData<UserModel>) -> impl Responder {
    // Return the user data
    HttpResponse::Ok().json(user.into_inner())
}

pub async fn logout(session: Session) -> impl Responder {
    // Remove the user_id from the session
    session.purge();

    HttpResponse::Ok().finish()
}
