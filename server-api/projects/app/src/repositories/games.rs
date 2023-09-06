use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};

use crate::entities::{game::Model as GameModel, prelude::*};
use crate::models::pagination::Paginated;

pub async fn get_games(db: &DbPool) -> AppResult<Vec<GameModel>> {
    let games = Game::find()
        .all(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(games)
}

pub async fn count_games(db: &DbPool) -> AppResult<u64> {
    let count = Game::find()
        .count(db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(count)
}

pub async fn paginate_games(
    db: &DbPool,
    page: u64,
    per_page: u64,
) -> AppResult<Paginated<GameModel>> {
    let games_paginator = Game::find().paginate(db, per_page);

    let counts = games_paginator.num_items_and_pages().await?;
    let games = games_paginator.fetch_page(page).await?;

    Ok(Paginated {
        data: games,
        current_page: page,
        total_pages: counts.number_of_pages,
        total_items: counts.number_of_items,
        per_page,
    })
}
