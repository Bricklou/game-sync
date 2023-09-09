use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};

use crate::entities::game;
use crate::entities::{game::Model as GameModel, prelude::*};
use crate::models::games::GameCreateInput;
use crate::models::pagination::{Paginated, Pagination, PaginationMeta};
use crate::models::search::Search;

pub async fn paginate_games(
    db: &DbPool,
    pagination_query: &Pagination,
    search_query: &Search,
) -> AppResult<Paginated<GameModel>> {
    let page = pagination_query.get_page();
    let per_page = pagination_query.get_per_page();

    tracing::debug!(
        "page: {}, per_page: {}, search_query: {:?}",
        page,
        per_page,
        search_query
    );

    // Find all games
    let mut game_query = Game::find();

    // Filter by search query
    if let Some(search) = search_query.get_search() {
        game_query = game_query.filter(
            Condition::any()
                .add(game::Column::Name.contains(search.clone()))
                .add(game::Column::Description.contains(search.clone())),
            // TODO: Add search by author
        );
    }

    game_query = game_query.order_by(game::Column::Name, search_query.get_sort_order().into());

    let games_paginator = game_query.paginate(db, per_page);

    let counts = games_paginator.num_items_and_pages().await?;

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
