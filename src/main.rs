use std::net::SocketAddr;

use tokio_rusqlite::Connection;

use crate::database::database_functions::{down, up};
use crate::routes::route_all::route;

mod error;
mod database;
mod routes;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main(){
    let conn = Connection::open("test.db").await.unwrap();

    // down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    let route_all = route(conn);
        // axum::Router::new()
        // .nest("/api", route(conn))
        // .route("/hello", axum::routing::get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
	axum::Server::bind(&addr)
		.serve(route_all.into_make_service())
		.await
		.unwrap();
}
