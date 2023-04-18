// use axum::{middleware::Next, response::Response, http::Request, extract::State};
// use tokio_rusqlite::Connection;

// use crate::error::AppError;

// pub async fn guard<T>(
//     State(database): State<Connection>,
//     TypedHeader(token): TypedHeader<String>,
// ) -> Result<Response, AppError> {

// }