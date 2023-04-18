use tokio_rusqlite::Connection;

pub struct AppState {
    pub db: Connection,
    pub session_token: i128,
}