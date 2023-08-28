use crate::entities::user::Model as UserModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: UserModel,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
