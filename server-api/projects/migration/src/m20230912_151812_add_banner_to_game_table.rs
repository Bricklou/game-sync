use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(BannerType::Type)
                    .values(BannerType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Banner::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Banner::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Banner::GameId).integer().not_null())
                    .col(
                        ColumnDef::new(Banner::BannerType)
                            .enumeration(BannerType::Type, BannerType::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Banner::Value).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_banner")
                            .from_col(Banner::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_game_banner")
                            .col(Banner::GameId)
                            .col(Banner::BannerType)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(BannerType::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Banner {
    Table,
    Id,
    GameId,
    BannerType,
    Value,
}

#[derive(Iden, EnumIter)]
pub enum BannerType {
    #[iden = "banner_type"]
    Type,
    #[iden = "Color"]
    Color,
    #[iden = "Image"]
    Image,
}
