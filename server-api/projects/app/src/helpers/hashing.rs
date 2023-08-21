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
pub fn verify_password(hash: &str, password: &String) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    let res = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    // Is there an error ? If yes, is it a verification error ?
    match res {
        Ok(_) => Ok(true),
        Err(e) => match e {
            argon2::password_hash::Error::Password => Ok(false),
            _ => Err(AppError::Argon2Error(e.into())),
        },
    }
}
