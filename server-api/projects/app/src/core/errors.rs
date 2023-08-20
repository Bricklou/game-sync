use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database Error")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Not Found Error")]
    NotFoundError,

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Unknown Error")]
    UnknownError,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFoundError => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.to_string(),
        })
    }
}
