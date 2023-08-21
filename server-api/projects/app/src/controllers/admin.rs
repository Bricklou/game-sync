use actix_web::{get, web};
use actix_web_lab::respond::Html;
use tera::Tera;

use crate::core::errors::AppResult;

#[get("")]
pub async fn index(tmpl: web::Data<Tera>) -> AppResult<Html> {
    let s = tmpl.render("index.html", &tera::Context::new())?;
    Ok(Html(s))
}
