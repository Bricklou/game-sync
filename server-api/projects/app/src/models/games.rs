use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GameCreateInput {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GameViewPath {
    #[validate(range(min = 1, message = "Game ID is required"))]
    pub id: i32,
}
