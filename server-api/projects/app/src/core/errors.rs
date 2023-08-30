use actix_web::{
    error::{JsonPayloadError, ResponseError},
    HttpRequest, HttpResponse,
};
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
    SessionGetError(#[from] actix_multi_session::SessionGetError),

    #[error("Set Session error: {0:?}")]
    SessionSetError(#[from] actix_multi_session::SessionInsertError),

    #[error("Save session error: {0:?}")]
    SessionSaveError(#[from] actix_multi_session::storage::SaveError),

    #[error("Load session error: {0:?}")]
    SessionLoadError(#[from] actix_multi_session::storage::LoadError),

    #[error("Already Exists: {0}")]
    AlreadyExists(String),

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
            AppError::AlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.to_string(),
        })
    }
}

#[derive(Serialize)]
struct ErrorJson {
    message: String,
}

#[tracing::instrument("json_error_handler", skip(err, _req))]
pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let details = ErrorJson {
        message: err.to_string(),
    };

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(details),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(details)
        }
        _ => HttpResponse::BadRequest().json(details),
    };

    actix_web::error::InternalError::from_response(err, resp).into()
}

#[tracing::instrument("validated_json_error_handler", skip(err, _req))]
pub fn validated_json_error_handler(
    err: actix_web_validator::Error,
    _req: &HttpRequest,
) -> actix_web::Error {
    use actix_web_validator::Error;

    let details = ErrorJson {
        message: err.to_string(),
    };

    let resp = match &err {
        Error::Deserialize(json_err) => {
            tracing::error!("Deserialize error: {}", json_err);
            HttpResponse::UnprocessableEntity().json(details)
        }
        actix_web_validator::Error::JsonPayloadError(JsonPayloadError::ContentType) => {
            HttpResponse::UnsupportedMediaType().json(details)
        }
        Error::Validate(json_err) => {
            tracing::error!("Validation error: {}", json_err);
            HttpResponse::UnprocessableEntity().json(details)
        }
        _ => HttpResponse::BadRequest().json(details),
    };

    actix_web::error::InternalError::from_response(err, resp).into()
}
