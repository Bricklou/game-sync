use actix_web_validator::{Json, Query};

pub type ValidatedJson<T> = Json<T>;
pub type ValidatedQuery<T> = Query<T>;
