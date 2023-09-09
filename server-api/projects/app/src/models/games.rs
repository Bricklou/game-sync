use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GameCreateInput {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub description: Option<String>,
}
