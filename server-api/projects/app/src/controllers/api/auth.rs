use std::collections::HashMap;

use actix_multi_session::{storage::SessionStore, Session};
use actix_web::{
    delete, get, post,
    web::{Data, ReqData},
    HttpResponse, Responder,
};
use time::Duration;

use crate::{
    core::{errors::AppError, types::ValidatedJson},
    data::AppData,
    entities::user::Model as UserModel,
    models::user::{UserLoginRequest, UserLoginResponse},
    repositories,
};

#[post("")]
pub async fn login(
    input: ValidatedJson<UserLoginRequest>,
    data: Data<AppData>,
) -> Result<impl Responder, AppError> {
    let user = repositories::user::login(&data.db, &input).await?;

    let session_state = HashMap::new();

    // create a session for the user
    let session_key = data
        .session_store
        .save(session_state, &Duration::weeks(7))
        .await
        .map_err(AppError::from)?;

    return Ok(HttpResponse::Ok().json(UserLoginResponse {
        token: session_key.into(),
        user,
    }));
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
