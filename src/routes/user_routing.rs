use axum::{Router, routing::post};
use tokio_rusqlite::Connection;

use crate::database::user::user_functions::{create_user, login};


pub fn user_route(db: Connection) -> Router {
    Router::new()
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        .with_state(db)
}