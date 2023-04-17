use axum::http::StatusCode;
use tokio_rusqlite::Connection;
use rusqlite::Connection as SqliteConnection;

use crate::database::event::event_structs::Event;
use crate::error::AppError;
use crate::database::event::event_structs::UpdateEvent;

impl From<tokio_rusqlite::Error> for AppError {
    fn from(error: tokio_rusqlite::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", error))
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", error))
    }
}

// NotFound
pub async fn up(conn: &Connection) -> Result<(),AppError> {
    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Events (
                id INTEGER PRIMARY KEY,
                start_time TEXT NOT NULL,
                end_time TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )
    }).await?;

    Ok(())
}

pub async fn down(conn: &Connection) -> Result<(),AppError> {
    conn.call(|conn| {
        conn.execute("DROP TABLE IF EXISTS Events;", [])
    }).await?;

    Ok(())
}

pub async fn db_insert_event(conn: &Connection, event: Event) -> Result<Event,AppError>{
    let return_event = conn.call(move |conn| {
        conn.execute(
            "INSERT INTO Events (id, start_time, end_time, title, description) values (?1, datetime(?2), datetime(?3), ?4, ?5)",
            &[&event.id.to_string(), &event.start_time, &event.end_time, &event.title, &event.description],
        )?;
        Ok::<_, rusqlite::Error>(event)
    }).await?;
    Ok(return_event)
}

pub async fn db_get_event(conn: &Connection, id: i32) -> Result<Event,AppError> {
    let return_event = conn.call( move |conn| {
        if let Ok(event) = _get_event(conn, id){
            Ok::<_, rusqlite::Error>(event)
        }
        else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }).await?;
    Ok(return_event)
}

pub async fn db_update_event(conn: &Connection, id: i32, update_event: UpdateEvent) -> Result<Event,AppError> {
    let return_event = conn.call(move |conn| {

        let mut stmt = conn.prepare(
            "UPDATE Events SET start_time = datetime(?2), end_time = datetime(?3), title = ?4, description = ?5 WHERE id = ?1",
        )?;

        let current_event = _get_event(conn, id).unwrap();

        stmt.execute(&[
            &id.to_string(),
            &update_event.start_time.unwrap_or(current_event.start_time).to_string(),
            &update_event.end_time.unwrap_or(current_event.end_time).to_string(),
            &update_event.title.unwrap_or(current_event.title),
            &update_event.description.unwrap_or(current_event.description),
        ])?;
        
        let event = _get_event(conn, id);

        Ok::<_, rusqlite::Error>(event)
    }).await?;

    Ok(return_event.unwrap())
}

pub async fn db_delete_event(conn: &Connection, id: i32) -> Result<Event,AppError> {
    let return_event = conn.call(move |conn| {

        let event = _get_event(conn, id);

        conn.execute("DELETE FROM Events WHERE id = ?1", &[&id])?;
        Ok::<_, rusqlite::Error>(event)

    }).await?;

    Ok(return_event.unwrap())
}

pub async fn db_list_events(conn: &Connection) -> Result<Vec<Event>,AppError> {
    let return_events = conn.call(|conn| {

        let mut stmt = conn.prepare("SELECT id, start_time, end_time, title, description FROM Events")?;

        let mut events = Vec::new();

        let events_iter = stmt.query_map([], |row| {
            Ok(Event {
                id: row.get(0)?,
                start_time: row.get(1)?,
                end_time: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
            })
        })?;

        for event in events_iter {
            events.push(event?);
        }

        Ok::<_, rusqlite::Error>(events)

    }).await?;
    Ok(return_events)
}

pub async fn db_get_next_id(conn: &Connection) -> Result<i32,AppError> {
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

fn _get_event(conn: &SqliteConnection, id: i32) -> Result<Event,AppError> {
    let mut stmt = conn.prepare("SELECT id, start_time, end_time, title, description FROM Events WHERE id = ?1")?;

    let event = stmt.query_row(&[&id], |row| {
        Ok(Event {
            id: row.get(0)?,
            start_time: row.get(1)?,
            end_time: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
        })
    })?;
    Ok(event)
}
