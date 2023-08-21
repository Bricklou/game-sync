use std::fmt::Debug;

use crate::helpers::validation::required_str::validate_required_str;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_required_str")]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserCreateInput {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_required_str")]
    pub password: String,
}
