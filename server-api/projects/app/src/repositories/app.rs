use super::user;
use crate::core::{database::DbPool, errors::AppError};

/// The application is configured only is there is at least 1 admin user to manage the app
pub async fn is_configured(db: &DbPool) -> Result<bool, AppError> {
    let user = user::count_users(db).await?;

    Ok(user > 0)
}
