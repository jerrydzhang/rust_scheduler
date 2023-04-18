use crate::error::AppError;

use axum::http::StatusCode;
use pbkdf2::{password_hash::{PasswordHasher, SaltString, PasswordVerifier, PasswordHash}, Pbkdf2};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Pbkdf2.hash_password(password.as_bytes(), &salt)
        .map_err(|_|{
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to hash password",
            )
        })?
        .to_string();

    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    let password_matched = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);
    match password_matched {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }        
}