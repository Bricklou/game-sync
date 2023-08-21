use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::core::errors::{AppError, AppResult};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub fn hash(password: &String) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let pass = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(pass)
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub async fn verify_password(hash: &str, password: &[u8]) -> AppResult<()> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .map_err(AppError::Argon2Error)
}
