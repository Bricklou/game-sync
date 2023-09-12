use actix_web::{web::Data, HttpResponse, Responder};

use crate::{
    core::{
        errors::{AppError, AppResult},
        types::{ValidatedJson, ValidatedPath, ValidatedQuery},
    },
    data::AppData,
    models::{
        games::{GameCreateInput, GameViewPath},
        pagination::Pagination,
        search::Search,
    },
    repositories,
};

#[tracing::instrument(name = "GET /api/games", skip(data))]
pub async fn get_games(
    pagination_query: ValidatedQuery<Pagination>,
    search_query: ValidatedQuery<Search>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let games =
        repositories::games::paginate_games(&data.db, &pagination_query, &search_query).await?;

    Ok(HttpResponse::Ok().json(games))
}

#[tracing::instrument(name = "POST /api/games", skip(data))]
pub async fn create_game(
    input: ValidatedJson<GameCreateInput>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let game = repositories::games::create_games(&data.db, &input).await?;

    Ok(HttpResponse::Ok().json(game))
}

#[tracing::instrument(name = "GET /api/games/{id}", skip(data))]
pub async fn get_game(
    path: ValidatedPath<GameViewPath>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let game = repositories::games::get_game(&data.db, path.into_inner().id).await?;

    if let Some(game) = game {
        return Ok(HttpResponse::Ok().json(game));
    }

    Err(AppError::NotFoundError)
}

#[tracing::instrument(name = "PUT /api/games/{id}", skip(data))]
pub async fn update_game(
    path: ValidatedPath<GameViewPath>,
    input: ValidatedJson<GameCreateInput>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let game = repositories::games::update_games(&data.db, path.into_inner().id, &input).await?;

    Ok(HttpResponse::Ok().json(game))
}
