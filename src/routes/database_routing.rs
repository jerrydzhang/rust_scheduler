use axum::extract::{State, Path};
use axum::{Router, Json, debug_handler};
use axum::routing::{get, post, put, delete};
use tokio_rusqlite::Connection;

use crate::database::{db_insert_event, db_list_events, db_get_next_id, db_get_event, db_update_event, db_delete_event};
use crate::events::{Event, CreateEvent, UpdateEvent};
use crate::error::AppError;



pub fn route(db: Connection) -> Router {
    Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route("/event", post(insert_event))
        .route("/event", get(list_events))
        .route("/event/:id", get(get_event))
        .route("/event/:id", put(update_event))
        .route("/event/:id", delete(delete_event))
        .with_state(db)
}

#[debug_handler]
async fn insert_event(
    State(conn): State<Connection>,
    Json(event): Json<CreateEvent>,
) -> Result<Json<Event>,AppError> {

    let id = db_get_next_id(&conn).await.unwrap();

    let event = db_insert_event(&conn, Event::new(id, event.start_time, event.end_time, event.title, event.description).await).await?;

    Ok(Json(event))
}

#[debug_handler]
async fn list_events(
    State(conn): State<Connection>
) -> Result<Json<Vec<Event>>,AppError> {
    let events = db_list_events(&conn).await?;

    Ok(Json(events))
}

#[debug_handler]
async fn get_event(
    Path(id): Path<u32>,
    State(conn): State<Connection>,
) -> Result<Json<Event>,AppError> {
    let event = db_get_event(&conn, id as i32).await?;

    Ok(Json(event))
}

#[debug_handler]
async fn update_event(
    Path(id): Path<u32>,
    State(conn): State<Connection>,
    Json(event): Json<UpdateEvent>,
) -> Result<Json<Event>,AppError> {
    let event = db_update_event(&conn, id as i32, event).await?;

    Ok(Json(event))
}

#[debug_handler]
async fn delete_event(
    Path(id): Path<u32>,
    State(conn): State<Connection>,
) -> Result<Json<Event>,AppError> {
    let event = db_delete_event(&conn, id as i32).await?;

    Ok(Json(event))
}

