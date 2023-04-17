// use axum::{Json, extract::State, http::StatusCode};
// use serde::Serialize;
// use tokio_rusqlite::Connection;

// use crate::database::user::user_structs::User;

// #[derive(Serialize)]
// pub struct RequestUser {
//     username: String,
//     password: String,
// }

// #[derive(Serialize)]
// pub struct ResponseUser{
//     username: String,
//     id: i32,
//     token: String,
// }

// pub async fn create_user(
//     State(conn): State<Connection>, 
//     Json(user): Json<RequestUser>
// ) -> Result<Json<ResponseUser>, StatusCode> {
//     User{
//         username: user.username,
//         password: user.password,
//         id: 0,
//         token: "".to_string(),
//     }
//     todo!()
// }