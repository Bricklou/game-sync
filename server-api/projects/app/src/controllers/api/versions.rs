use crate::{
    core::{
        errors::AppResult,
        types::{ValidatedJson, ValidatedPath, ValidatedQuery},
    },
    models::games::GameViewPath,
};

use actix_web::{
    web::{Data, Redirect},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::data::AppData;

#[derive(Debug, Deserialize, Validate)]
pub struct UploadRequest {
    file_size: usize,
    filename: String,
}

#[tracing::instrument("POST /api/games/{id}/versions", skip(data, path), fields(id = %path.id))]
pub async fn upload_file(
    query_data: ValidatedQuery<UploadRequest>,
    path: ValidatedPath<GameViewPath>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let s3client = data.s3.clone();

    let prefix = format!("uploads/images/");

    let presigned_url = s3client
        .create_presigned_url(&query_data.filename, query_data.file_size, &prefix)
        .await?;

    return Ok(HttpResponse::Ok().json(presigned_url));
}

#[tracing::instrument("GET /api/games/{id}/versions")]
pub async fn get_versions(path: ValidatedPath<GameViewPath>) -> AppResult<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}
