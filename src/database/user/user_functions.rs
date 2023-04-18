use axum::{Json, extract::State, http::StatusCode, debug_handler};
use serde::{Serialize, Deserialize};
use tokio_rusqlite::Connection;
use rusqlite::Connection as RusqliteConnection;

use crate::database::user::user_structs::User;
use crate::error::AppError;

use super::pwd_hash::{hash_password, verify_password};

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser{
    id: i32,
    username: String,
    token: String,
}

#[debug_handler]
pub async fn create_user(
    State(conn): State<Connection>, 
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>,AppError> {

    let hashed_password = hash_password(&request_user.password)?;

    let new_id = conn.call(|conn| {

        let mut stmt = conn.prepare(
        "SELECT DISTINCT IFNULL(min(id + 1),0)
            FROM Users
            WHERE id + 1 NOT IN (SELECT DISTINCT id FROM Users);"
        )?;

        let id: i32 = stmt.query_row([], |row| row.get(0))?;

        Ok(id)
    }).await?;

    let user = User
    {
        // implement id generation
        id: new_id,
        username: request_user.username,
        // implement password hashing
        password: hashed_password,
        // implement token generation
        token: "".to_string(),
    };

    let response_user = conn.call(move |conn|{
        conn.execute(
            "INSERT INTO Users (id, username, password, token) values (?1, ?2, ?3, ?4)",
            &[&user.id.to_string(), &user.username, &user.password, &user.token],
        )?;

        let response_user = _get_user_by_username(conn, user.username)?;

        Ok::<_, rusqlite::Error>(response_user)
    }).await?;

    Ok(Json(response_user))
}

#[debug_handler]
pub async fn login(
    State(conn): State<Connection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<User>,AppError> {
    let db_user = conn.call(move |conn|{
        let response_user = _get_user_by_username(conn, request_user.username.clone())?;
        let hashed_password = _get_hashed_password_by_username(conn, request_user.username.clone())?;

        let db_user = User::new(response_user.id, response_user.username, hashed_password, response_user.token);
        
        Ok::<_, rusqlite::Error>(db_user)
    }).await;

    match db_user {
        Ok(db_user) => {
            if verify_password(&request_user.password, &db_user.password)? {
                // generate token and update db
                return Ok(Json(db_user));
            } else {
                return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
            }
        },
        Err(_) => return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid username".to_string()))
    }

}


pub fn _get_user_by_username(conn: &RusqliteConnection, username: String) -> Result<ResponseUser,rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, token FROM Users WHERE username = ?1",
    )?;

    let response_user = stmt.query_row(&[&username], |row| {
        Ok(ResponseUser {
            id: row.get(0)?,
            username: row.get(1)?,
            token: row.get(3)?,
        })
    })?;

    Ok(response_user)
}

pub fn _get_hashed_password_by_username(conn: &RusqliteConnection, username: String) -> Result<String,rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT password FROM Users WHERE username = ?1",
    )?;

    let hashed_password = stmt.query_row(&[&username], |row| {
        Ok(row.get(0)?)
    })?;

    Ok(hashed_password)
}