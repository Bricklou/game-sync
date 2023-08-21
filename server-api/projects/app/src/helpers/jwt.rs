use super::jwt_numeric_date;
use crate::{core::errors::AppResult, entities::user::Model as UserModel};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl Claims {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `OffsetDateTime` contains a microsecond
    /// field but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes
    /// the timestamps.
    pub fn new(sub: String, iat: OffsetDateTime, duration: Duration) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();

        let exp = iat + duration;
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();

        Self { sub, iat, exp }
    }
}

#[tracing::instrument(name = "Generating tokens", skip(user, secret))]
pub fn generate_tokens(user: &UserModel, duration: Duration, secret: &String) -> AppResult<String> {
    let sub = user.id.to_string();
    let iat = OffsetDateTime::now_utc();

    let claims = Claims::new(sub, iat, duration);

    let tokens = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(tokens)
}

#[tracing::instrument(name = "Validating token", skip(token, secret))]
pub fn validate_token(token: &String, secret: &String) -> AppResult<Claims> {
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

#[tracing::instrument(name = "Generating tokens response", skip(user, secret))]
pub fn generate_tokens_response(user: &UserModel, secret: &String) -> AppResult<TokenResponse> {
    let access_token = generate_tokens(user, Duration::minutes(15), secret)?;
    let refresh_token = generate_tokens(user, Duration::days(7), secret)?;

    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}
