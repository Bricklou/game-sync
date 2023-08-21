use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};
use crate::entities::user::Model as UserModel;
use crate::entities::{prelude::*, user};
use crate::helpers::hashing;
use crate::models::user::UserLoginRequest;

pub async fn get_users(db: &DbPool) -> AppResult<Vec<UserModel>> {
    let users = User::find()
        .all(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(users)
}

pub async fn get_user_from_id(db: &DbPool, id: i32) -> AppResult<Option<UserModel>> {
    let user = User::find_by_id(id)
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(user)
}

pub async fn get_user_from_email(db: &DbPool, email: &String) -> AppResult<Option<UserModel>> {
    let user = User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(user)
}

pub async fn login(db: &DbPool, login_input: &UserLoginRequest) -> AppResult<Option<UserModel>> {
    let user = get_user_from_email(db, &login_input.email).await?;

    if let Some(user) = user {
        let hashed_pass = hashing::hash(&login_input.password)?;

        if user.password == hashed_pass {
            return Ok(Some(user));
        }
    }

    Ok(None)
}
