use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: i32,
    pub start_time: i32,
    pub end_time: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateEvent{
    pub start_time: i32,
    pub end_time: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateEvent{
    pub start_time: Option<i32>,
    pub end_time: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Event {
    pub async fn new(
        id: i32,
        start_time: i32,
        end_time: i32,
        title: String,
        description: String,
    ) -> Self {
        Event {
            id,
            start_time,
            end_time,
            title,
            description,
        }
    }
}

#[cfg(test)]
impl UpdateEvent{
    pub async fn new(
        start_time: Option<i32>,
        end_time: Option<i32>,
        title: Option<String>,
        description: Option<String>,
    ) -> Self {
        UpdateEvent {
            start_time,
            end_time,
            title,
            description,
        }
    }
}
