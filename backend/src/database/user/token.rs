
use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio_rusqlite::Connection;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn generate_token(secret: &str, username: String) -> Result<String, AppError> {
    // add at least an hour for this timestamp
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })
        .map(|_claim| true)
}

pub async fn set_token(conn: Connection, username: String, token: String) -> Result<String, AppError> {
    let return_token = token.clone();
    let token_success = conn.call(move |conn|{
        conn.execute("UPDATE users SET token = ?1 WHERE username = ?2", &[&token, &username])?;

        Ok::<_, rusqlite::Error>(())
    }).await;
    match token_success {
        Ok(_) => Ok(return_token),
        Err(_) => Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error setting token")),
    }
}