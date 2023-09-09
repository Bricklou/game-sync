use actix_web_validator::{Json, Path, Query};

pub type ValidatedJson<T> = Json<T>;
pub type ValidatedQuery<T> = Query<T>;
pub type ValidatedPath<T> = Path<T>;
