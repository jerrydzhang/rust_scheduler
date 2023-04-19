use axum::{Router, routing::{get, post, put, delete}, middleware};
use tokio_rusqlite::Connection;
use tower_cookies::CookieManagerLayer;

use crate::database::user::{user_functions::{create_user, login, logout}};
use crate::database::event::event_functions::{insert_event, list_events, get_event, update_event, delete_event};

use super::guard::guard;



pub fn route(db: Connection) -> Router {
    Router::new()
        .route("/api/event", post(insert_event))
        .route("/api/event", get(list_events))
        .route("/api/event/:id", get(get_event))
        .route("/api/event/:id", put(update_event))
        .route("/api/event/:id", delete(delete_event))
        .route_layer(middleware::from_fn_with_state(db.clone(), guard))
        .route("/api/create_user", post(create_user))
        .route("/api/login", post(login))
        .route("/api/logout",post(logout))
        .route("/api/hello", get(|| async { "Hello, World!" }))
        .with_state(db)
        .layer(CookieManagerLayer::new())
}