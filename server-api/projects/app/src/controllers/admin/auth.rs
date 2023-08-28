use actix_session::Session;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    core::{
        errors::{AppError, AppResult},
        types::ValidatedJson,
    },
    data::AppData,
    models::user::{UserCreateInput, UserLoginRequest},
    repositories,
};

pub async fn register(
    form_data: ValidatedJson<UserCreateInput>,
    app_data: Data<AppData>,
    session: Session,
) -> AppResult<impl Responder> {
    let user = repositories::user::create_user(&app_data.db, &form_data).await?;

    // Create a session for the user
    session.insert("user_id", user.id)?;

    // Return the user
    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(
    form_data: ValidatedJson<UserLoginRequest>,
    app_data: web::Data<AppData>,
    session: Session,
) -> AppResult<impl Responder> {
    let user = repositories::user::login(&app_data.db, &form_data).await?;

    // Create a session for the user
    session.insert("user_id", user.id)?;

    // Return the user
    Ok(HttpResponse::Ok().json(user))
}

#[tracing::instrument("GET /admin/auth/me", skip(session, app_data))]
pub async fn me(session: Session, app_data: web::Data<AppData>) -> AppResult<impl Responder> {
    let user_id = session.get::<i32>("user_id")?;

    tracing::trace!("User id: {:?}", user_id);

    match user_id {
        Some(user_id) => {
            // Unwrap because we know that we have the data, otherwise that something is wrong
            let user = repositories::user::get_user_from_id(&app_data.db, user_id).await?;

            if user.is_none() {
                return Err(AppError::Unauthorized);
            }

            Ok(HttpResponse::Ok().json(user))
        }
        None => Err(AppError::Unauthorized),
    }
}

#[tracing::instrument("DELETE /admin/auth", skip(session))]
pub async fn logout(session: Session) -> AppResult<impl Responder> {
    session.remove("user_id");

    Ok(HttpResponse::Ok().finish())
}
