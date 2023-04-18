use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};
use tokio_rusqlite::Connection;

use crate::database::{database_functions::{down, up}, user::user_structs::User};
use crate::routes::route_all::route;

mod error;
mod database;
mod routes;
mod app_state;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main(){
    let conn = Connection::open("test.db").await.unwrap();

    // down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    let user = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 2,
        token: "testtoken".to_string(),
    };

    let route_all = route(conn, user);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
	axum::Server::bind(&addr)
		.serve(route_all.into_make_service())
		.await
		.unwrap();
}
