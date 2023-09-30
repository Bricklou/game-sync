use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

use crate::helpers::colors;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "banner_type")]
#[serde(rename_all = "snake_case")]
pub enum BannerType {
    #[sea_orm(string_value = "Color")]
    Color,
    #[sea_orm(string_value = "Image")]
    Image,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "banner")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub banner_type: BannerType,
    pub value: String,
    pub game_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::game::Entity",
        from = "Column::GameId",
        to = "super::game::Column::Id"
    )]
    Game,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
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
}

impl ActiveModel {
    /// Default from game
    pub fn from_game(game: &super::game::Model) -> Self {
        Self {
            game_id: Set(game.id.clone()),
            banner_type: Set(BannerType::Color),
            value: Set(colors::Color::from_text(game.name.clone()).to_string()),
            ..Self::new()
        }
    }
}
