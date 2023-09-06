use actix_web::{web::Data, HttpResponse, Responder};

use crate::{
    core::{errors::AppError, types::ValidatedQuery},
    data::AppData,
    models::pagination::Pagination,
    repositories,
};

pub async fn get_games(
    query: ValidatedQuery<Pagination>,
    data: Data<AppData>,
) -> Result<impl Responder, AppError> {
    let games =
        repositories::games::paginate_games(&data.db, query.get_page(), query.get_per_page())
            .await?;

    Ok(HttpResponse::Ok().json(games))
}

pub async fn create_game() -> impl Responder {
    HttpResponse::Ok().body("create_game")
}
