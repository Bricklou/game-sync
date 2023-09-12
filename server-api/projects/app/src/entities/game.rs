use crate::helpers;

use super::game_banner::{self, Entity as GameBanner};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::game_banner::Entity")]
    GameBanner,
}

impl Related<super::game_banner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameBanner.def()
    }
}

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
            let now = OffsetDateTime::now_utc();
            this.updated_at = Set(now);
        }

        Ok(this)
    }

    /// Will be triggered after insert / update
    async fn after_save<C>(game: Model, db: &C, _insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        {
            // Check if the game has a banner
            let banner = GameBanner::find()
                .filter(game_banner::Column::GameId.eq(game.id))
                .one(db)
                .await?;

            if banner.is_none() {
                // If the game doesn't have a banner, create one
                let color = helpers::colors::Color::from_text(game.name.clone());

                let banner = game_banner::ActiveModel {
                    game_id: Set(game.id),
                    banner_type: Set(game_banner::BannerType::Color),
                    value: Set(color.to_string()),
                    ..Default::default()
                };

                banner.insert(db).await?;

                tracing::info!("Created banner for game {}", game.name);
            }
        }

        Ok(game)
    }
}
