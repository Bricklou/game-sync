use sea_orm::EntityTrait;

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};
use crate::entities::prelude::*;
use crate::entities::user::Model as UserModel;

pub async fn get_users(db: &DbPool) -> AppResult<Vec<UserModel>> {
    let users = User::find()
        .all(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(users)
}

pub async fn get_user(db: &DbPool, id: i32) -> AppResult<Option<UserModel>> {
    let user = User::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(user)
}
