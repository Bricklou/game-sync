use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Not Found Error")]
    NotFoundError,

    #[error("Failed to parse IP Address: {0}")]
    IpAddrError(#[from] std::net::AddrParseError),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Argon2 Error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("Tera Error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("Internal Error")]
    InternalError,

    #[error("Get Session error: {0:?}")]
    SessionGetError(#[from] actix_session::SessionGetError),

    #[error("Set Session error: {0:?}")]
    SessionSetError(#[from] actix_session::SessionInsertError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unknown Error")]
    UnknownError,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFoundError => actix_web::http::StatusCode::NOT_FOUND,
            AppError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.to_string(),
        })
    }
}
