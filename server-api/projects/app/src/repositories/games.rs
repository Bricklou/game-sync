use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::core::database::DbPool;
use crate::core::errors::{AppError, AppResult};

use crate::entities::{game, game_banner};
use crate::entities::{
    game::Model as GameModel, game_banner::Model as GameBannerModel, prelude::*,
};
use crate::helpers;
use crate::models::games::{GameCreateInput, GameGetResponse};
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

pub async fn create_games(db: &DbPool, game_input: &GameCreateInput) -> AppResult<GameModel> {
    let game = game::ActiveModel {
        name: Set(game_input.name.clone()),
        description: Set(game_input.description.clone()),
        ..Default::default()
    };

    let game = game.insert(db).await.map_err(AppError::DatabaseError)?;

    if let Some(banner_input) = &game_input.banner_type {
        create_game_banner(
            db,
            &game,
            banner_input.banner_type.clone(),
            banner_input.value.clone().unwrap_or_default(),
        )
        .await?;
    } else {
        create_default_game_banner(db, &game).await?;
    }

    Ok(game)
}

pub async fn create_default_game_banner(
    db: &DbPool,
    game_obj: &GameModel,
) -> AppResult<GameBannerModel> {
    let color = helpers::colors::Color::from_text(game_obj.name.clone());

    create_game_banner(
        db,
        game_obj,
        game_banner::BannerType::Color,
        color.to_string(),
    )
    .await
}

pub async fn create_game_banner(
    db: &DbPool,
    game_obj: &GameModel,
    banner_type: game_banner::BannerType,
    value: String,
) -> AppResult<GameBannerModel> {
    let banner = game_banner::ActiveModel {
        game_id: Set(game_obj.id),
        banner_type: Set(banner_type),
        value: Set(value),
        ..Default::default()
    };

    let banner = banner.insert(db).await.map_err(AppError::DatabaseError)?;

    Ok(banner)
}

pub async fn get_game(db: &DbPool, id: i32) -> AppResult<Option<GameGetResponse>> {
    let game = Game::find_by_id(id)
        .find_also_related(GameBanner)
        .one(db)
        .await
        .map_err(AppError::DatabaseError)?
        .map_or(None, |(game, banner)| {
            // Our game model
            Some(GameGetResponse {
                id: game.id,
                name: game.name,
                description: game.description,

                // Our game banner model
                banner: banner.map_or(None, |banner| {
                    Some(GameBannerModel {
                        id: banner.id,
                        banner_type: banner.banner_type,
                        value: banner.value,
                        game_id: banner.game_id,
                    })
                }),
            })
        });

    Ok(game)
}
