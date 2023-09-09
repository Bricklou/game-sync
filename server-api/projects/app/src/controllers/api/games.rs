use actix_web::{web::Data, HttpResponse, Responder};

use crate::{
    core::{
        errors::AppResult,
        types::{ValidatedJson, ValidatedQuery},
    },
    data::AppData,
    models::{games::GameCreateInput, pagination::Pagination, search::Search},
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

pub async fn create_game(
    input: ValidatedJson<GameCreateInput>,
    data: Data<AppData>,
) -> AppResult<impl Responder> {
    let game = repositories::games::create_games(&data.db, &input).await?;

    Ok(HttpResponse::Ok().json(game))
}
