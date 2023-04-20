use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: i32,
    pub user_id: i32,
    pub start_time: String,
    pub end_time: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateEvent{
    pub start_time: String,
    pub end_time: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateEvent{
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}