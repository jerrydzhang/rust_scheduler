use std::fmt;

use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct AppError {
    description: String,
    status_code: StatusCode,
}

impl AppError {
    pub fn new(status_code: StatusCode, description: impl Into<String>) -> Self {
        Self {
            status_code,
            description: description.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(ErrorResponse {
                error: self.description.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}