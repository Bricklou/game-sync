use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::helpers::hashing;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(column_name = "_password")]
    #[serde(skip)]
    pub password: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Create a new ActiveModel with default values. Also used by `Default::default()`.
    fn new() -> Self {
        Self {
            ..ActiveModelTrait::default()
        }
    }

    /// Will be triggered before insert / update
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;

        {
            this.email = Set(this.email.as_ref().to_lowercase());
        }

        {
            let hashed_password = hashing::hash(this.password.as_ref())
                .map_err(|_| DbErr::Custom("[before_save] Failed to hash password".to_string()))?;

            this.password = Set(hashed_password);
        }

        {
            let now = OffsetDateTime::now_utc();
            this.updated_at = Set(now);
        }

        Ok(this)
    }
}
