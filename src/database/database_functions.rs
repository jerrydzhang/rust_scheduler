use tokio_rusqlite::Connection;

use crate::error::AppError;

pub async fn up(conn: &Connection) -> Result<(),AppError> {
    conn.call(|conn| {
        conn.execute_batch(
            "BEGIN;
                CREATE TABLE IF NOT EXISTS Users (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    password TEXT NOT NULL,
                    token TEXT NOT NULL
                );
                CREATE TABLE IF NOT EXISTS Events (
                    id INTEGER PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    start_time TEXT NOT NULL,
                    end_time TEXT NOT NULL,
                    title TEXT NOT NULL,
                description TEXT NOT NULL
                );
                COMMIT;",
        )
    }).await?;

    Ok(())
}

pub async fn down(conn: &Connection) -> Result<(),AppError> {
    conn.call(|conn| {
        conn.execute_batch(
        "BEGIN;
            DROP TABLE IF EXISTS Events;
            DROP TABLE IF EXISTS Users;
            COMMIT;"
        )
    }).await?;

    Ok(())
}