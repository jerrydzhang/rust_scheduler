use axum::{Router, routing::{get, post, put, delete}, Extension};
use tokio_rusqlite::Connection;

use crate::database::user::{user_structs::User, user_functions::{create_user, login}};
use crate::database::event::event_functions::{insert_event, list_events, get_event, update_event, delete_event};



pub fn route(db: Connection, user: User) -> Router {
    Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route("/event", post(insert_event))
        .route("/event", get(list_events))
        .route("/event/:id", get(get_event))
        .route("/event/:id", put(update_event))
        .route("/event/:id", delete(delete_event))
        .layer(Extension(user))
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        .with_state(db)
}