use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};

use crate::entities::game;
use crate::entities::{game::Model as GameModel, prelude::*};
use crate::models::games::GameCreateInput;
use crate::models::pagination::{Paginated, PaginationMeta};

pub async fn paginate_games(
    db: &DbPool,
    page: u64,
    per_page: u64,
) -> AppResult<Paginated<GameModel>> {
    let games_paginator = Game::find().paginate(db, per_page);

    let counts = games_paginator.num_items_and_pages().await?;

    if counts.number_of_pages <= page {
        return Err(AppError::NotFound);
    }

    let games = games_paginator.fetch_page(page).await?;

    Ok(Paginated {
        data: games,
        meta: PaginationMeta {
            current_page: page,
            total_pages: counts.number_of_pages,
            total_items: counts.number_of_items,
            per_page,
        },
    })
}

pub async fn create_games(db: &DbPool, game: &GameCreateInput) -> AppResult<GameModel> {
    let game = game::ActiveModel {
        name: Set(game.name.clone()),
        description: Set(game.description.clone()),
        ..Default::default()
    };

    let game = game.insert(db).await.map_err(AppError::DatabaseError)?;

    Ok(game)
}
