use actix_web::{get, web};
use actix_web_lab::respond::Html;

use crate::{core::errors::AppResult, data::AppData};

#[get("")]
pub async fn index(app_data: web::Data<AppData>) -> AppResult<Html> {
    let s = app_data.tera.render("api.html", &tera::Context::new())?;
    Ok(Html(s))
}
