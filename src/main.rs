use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};
use tokio_rusqlite::Connection;

use crate::database::{down, up};

mod error;
mod database;
mod events;
mod routes;
#[cfg(test)]
mod test;

#[tokio::main]
async fn main(){
    let conn = Connection::open("test.db").await.unwrap();

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    let route_all = Router::new()
        .nest("/api", routes::database_routing::route(conn))
        .route("/", get(|| async { Html("<h1>Hello, World!</h1>") }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
	axum::Server::bind(&addr)
		.serve(route_all.into_make_service())
		.await
		.unwrap();
}
