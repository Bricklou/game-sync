use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use tauri::{plugin::TauriPlugin, Runtime};

pub fn register<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_stronghold::Builder::new(|password| {
        // TODO: hash the password here with e.g. argon2, blake2b or any other secure algorithm

        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
            .into_bytes()
    })
    .build()
}
