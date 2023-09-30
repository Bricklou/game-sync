use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::Either;
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

use crate::entities::game_banner::BannerType;
use crate::entities::game_banner::Model as GameBannerModel;

#[derive(Debug, Deserialize, Validate)]
pub struct GameCreateInput {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub description: Option<String>,
    pub banner_type: Option<GameBannerCreateInput>,
}

#[derive(Debug, Serialize)]
pub struct GameGetResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub banner: Option<GameBannerModel>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GameBannerCreateInput {
    pub banner_type: BannerType,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GameViewPath {
    #[validate(range(min = 1, message = "Game ID is required"))]
    pub id: i32,
}

#[derive(MultipartForm)]
#[multipart(deny_unknown_fields)]
pub struct GameBannerImageUpload {
    #[multipart(limit = "2 MiB", rename = "value")]
    pub image: TempFile,
}

#[derive(MultipartForm)]
pub struct GameBannerColorUpload {
    pub color: Text<String>,
}

pub type GameBannerUpload =
    Either<MultipartForm<GameBannerImageUpload>, MultipartForm<GameBannerColorUpload>>;
