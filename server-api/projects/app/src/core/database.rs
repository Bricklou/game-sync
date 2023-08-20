use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use super::errors::{AppError, AppResult};

pub type DbPool = DatabaseConnection;

pub async fn init_pool(database_url: &str) -> AppResult<DbPool> {
    let opt = ConnectOptions::new(database_url);

    Database::connect(opt)
        .await
        .map_err(AppError::DatabaseError)
}
