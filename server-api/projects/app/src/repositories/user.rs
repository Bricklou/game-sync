use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set,
};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};
use crate::entities::user::Model as UserModel;
use crate::entities::{prelude::*, user};
use crate::helpers::hashing;
use crate::models::user::{UserCreateInput, UserLoginRequest};

pub async fn get_users(db: &DbPool) -> AppResult<Vec<UserModel>> {
    let users = User::find()
        .all(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(users)
}

pub async fn count_users(db: &DbPool) -> AppResult<u64> {
    let count = User::find()
        .count(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(count)
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

#[tracing::instrument(name = "Login user", skip(login_input, db))]
pub async fn login(db: &DbPool, login_input: &UserLoginRequest) -> AppResult<UserModel> {
    let user = get_user_from_email(db, &login_input.email).await?;

    if let Some(user) = user {
        if hashing::verify_password(&user.password, &login_input.password)? {
            return Ok(user);
        }
    }

    Err(AppError::Unauthorized)
}

#[tracing::instrument(name = "Create user", skip(user, db))]
pub async fn create_user(db: &DbPool, user: &UserCreateInput) -> AppResult<UserModel> {
    let user = user::ActiveModel {
        email: Set(user.email.clone()),
        password: Set(user.password.clone()),
        ..Default::default()
    };

    let user = user.insert(db).await.map_err(AppError::DatabaseError)?;

    Ok(user)
}
