use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{models::user::UserCreateInput, repositories};

use super::errors::{AppError, AppResult};

pub type DbPool = DatabaseConnection;

#[tracing::instrument("initialize database", skip(database_url))]
pub async fn init_pool(database_url: &str) -> AppResult<DbPool> {
    let opt = ConnectOptions::new(database_url);

    Database::connect(opt)
        .await
        .map_err(AppError::DatabaseError)
}

#[tracing::instrument("seeding database", skip(pool))]
pub async fn seed_database(pool: &DbPool) -> AppResult<()> {
    let user_count = repositories::user::count_users(pool).await?;

    if user_count > 0 {
        return Ok(());
    }

    let user = UserCreateInput {
        email: "admin@admin.com".to_owned(),
        password: "password".to_owned(),
    };

    repositories::user::create_user(pool, &user).await?;

    Ok(())
}
