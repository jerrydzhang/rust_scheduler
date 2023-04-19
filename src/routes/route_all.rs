use axum::{Router, routing::{get, post, put, delete}, Extension, middleware};
use tokio_rusqlite::Connection;
use tower_cookies::CookieManagerLayer;

use crate::database::user::{user_structs::User, user_functions::{create_user, login, logout}};
use crate::database::event::event_functions::{insert_event, list_events, get_event, update_event, delete_event};

use super::guard::guard;



pub fn route(db: Connection, user: User) -> Router {
    Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route("/event", post(insert_event))
        .route("/event", get(list_events))
        .route("/event/:id", get(get_event))
        .route("/event/:id", put(update_event))
        .route("/event/:id", delete(delete_event))
        .route_layer(middleware::from_fn_with_state(db.clone(), guard))
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        .route("/logout",get(logout))
        .with_state(db)
        .layer(CookieManagerLayer::new())
}