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

#[cfg(test)]
impl Event {
    pub async fn new(
        id: i32,
        user_id: i32,
        start_time: String,
        end_time: String,
        title: String,
        description: String,
    ) -> Self {
        Event {
            id,
            user_id,
            start_time,
            end_time,
            title,
            description,
        }
    }
}

#[cfg(test)]
impl CreateEvent {
    pub async fn new(
        start_time: String,
        end_time: String,
        title: String,
        description: String,
    ) -> Self {
        CreateEvent {
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
        start_time: Option<String>,
        end_time: Option<String>,
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
