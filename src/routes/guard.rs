use axum::{middleware::Next, response::Response, http::{Request, HeaderMap, self, StatusCode}, extract::State, TypedHeader, debug_handler};
use tokio_rusqlite::Connection;
use tower_cookies::Cookies;

use crate::{error::AppError, database::user::{token::validate_token, user_functions::get_user_by_token}};


pub async fn guard<T>(
    State(conn): State<Connection>,
    cookies: Cookies,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let cookie = cookies.get("token");
    let token = cookie.map(|cookie| cookie.value().to_string());

    match token.clone() {
        Some(token) => {
            validate_token("secret", token.as_str())?;
        }
        None => {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "No token".to_string(),
            ));
        }
    }
    
    let user = get_user_by_token(conn, token.unwrap()).await?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)

    // match token {
    //     Some(token) => {
    //         if validate_token("secret", token.as_str()).unwrap() {
    //         }
    //         else {
    //             return Err(AppError::new(
    //                 StatusCode::UNAUTHORIZED,
    //                 "Invalid token".to_string(),
    //             ));
    //         }
    //     },
    //     None => {
    //         return Err(AppError::new(
    //             StatusCode::UNAUTHORIZED,
    //             "No token".to_string(),
    //         ));
    //     }
    // }
}