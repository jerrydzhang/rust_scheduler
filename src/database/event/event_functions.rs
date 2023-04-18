use tokio_rusqlite::Connection;
use rusqlite::Connection as SqliteConnection;

use crate::database::event::event_structs::Event;
use crate::error::AppError;
use crate::database::event::event_structs::UpdateEvent;
use crate::database::user::user_structs::User;

use super::event_structs::CreateEvent;

use axum::extract::{State, Path};
use axum::{Json, debug_handler, Extension};

#[debug_handler]
pub async fn insert_event(
    State(conn): State<Connection>,
    Extension(user): Extension<User>,
    Json(create_event): Json<CreateEvent>,
) -> Result<Json<Event>,AppError> {
    let event_id = _db_get_next_id(&conn).await?;
    let user_id = user.id;
    let event = conn.call(move |conn| {
        conn.execute(
            "INSERT INTO Events (id, user_id, start_time, end_time, title, description) values (?1, ?2, datetime(?3), datetime(?4), ?5, ?6)",
            &[&event_id.to_string(), &user_id.to_string(), &create_event.start_time, &create_event.end_time, &create_event.title, &create_event.description],
        )?;
        let return_event = _get_event(conn, user_id, event_id)?;
        Ok::<_, rusqlite::Error>(return_event)
    }).await?;
    Ok(Json(event))
}

#[debug_handler]
pub async fn list_events(
    State(conn): State<Connection>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<Event>>,AppError> {
    let user_id = user.id;
    let events = conn.call(move |conn| {

        let mut stmt = conn.prepare("SELECT id, user_id, start_time, end_time, title, description FROM Events Where user_id = ?1")?;

        let mut events = Vec::new();

        let events_iter = stmt.query_map(&[&user_id], |row| {
            Ok(Event {
                id: row.get(0)?,
                user_id: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get(3)?,
                title: row.get(4)?,
                description: row.get(5)?,
            })
        })?;

        for event in events_iter {
            events.push(event?);
        }

        Ok::<_, rusqlite::Error>(events)

    }).await?;
    Ok(Json(events))
}

#[debug_handler]
pub async fn get_event(
    Path(id): Path<i32>,
    State(conn): State<Connection>,
    Extension(user): Extension<User>,
) -> Result<Json<Event>,AppError> {
    let user_id = user.id;
    let event = conn.call( move |conn| {
        let return_event = _get_event(conn, user_id, id)?;
        Ok::<_, rusqlite::Error>(return_event)
    }).await?;
    Ok(Json(event))
}

#[debug_handler]
pub async fn update_event(
    Path(id): Path<i32>,
    State(conn): State<Connection>,
    Extension(user): Extension<User>,
    Json(update_event): Json<UpdateEvent>,
) -> Result<Json<Event>,AppError> {
    let user_id = user.id;
    let event = conn.call(move |conn| {

        let mut stmt = conn.prepare(
            "UPDATE Events SET start_time = datetime(?3), end_time = datetime(?4), title = ?5, description = ?6 WHERE id = ?1 AND user_id = ?2",
        )?;

        let current_event = _get_event(conn, user_id, id)?;

        stmt.execute(&[
            &id.to_string(),
            &user_id.to_string(),
            &update_event.start_time.unwrap_or(current_event.start_time).to_string(),
            &update_event.end_time.unwrap_or(current_event.end_time).to_string(),
            &update_event.title.unwrap_or(current_event.title),
            &update_event.description.unwrap_or(current_event.description),
        ])?;
        
        let event = _get_event(conn, user_id, id)?;

        Ok::<_, rusqlite::Error>(event)
    }).await?;
    Ok(Json(event))
}

#[debug_handler]
pub async fn delete_event(
    Path(id): Path<i32>,
    State(conn): State<Connection>,
    Extension(user): Extension<User>,
) -> Result<Json<Event>,AppError> {
    let user_id = user.id;
    let event = conn.call(move |conn| {

        let event = _get_event(conn, user_id, id)?;

        conn.execute("DELETE FROM Events WHERE id = ?1 AND user_id = ?2", &[&id,&user_id])?;
        Ok::<_, rusqlite::Error>(event)

    }).await?;
    Ok(Json(event))
}

pub async fn _db_get_next_id(conn: &Connection) -> Result<i32,AppError> {
    let return_id = conn.call(|conn| {

        let mut stmt = conn.prepare(
        "SELECT DISTINCT IFNULL(min(id + 1),0)
            FROM Events
            WHERE id + 1 NOT IN (SELECT DISTINCT id FROM Events);"
        )?;

        let id: i32 = stmt.query_row([], |row| row.get(0))?;

        Ok(id)
    
    }).await?;
    Ok(return_id)
}

fn _get_event(conn: &SqliteConnection, user_id: i32, id: i32) -> Result<Event,rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, user_id, start_time, end_time, title, description FROM Events WHERE id = ?1 AND user_id = ?2")?;

    let event = stmt.query_row(&[&id,&user_id], |row| {
        Ok(Event {
            id: row.get(0)?,
            user_id: row.get(1)?,
            start_time: row.get(2)?,
            end_time: row.get(3)?,
            title: row.get(4)?,
            description: row.get(5)?,
        })
    })?;
    Ok(event)
}


