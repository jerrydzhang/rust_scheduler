use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub token: String,
}

impl User {
    pub fn new(id: i32, username: String, password: String, token: String) -> Self {
        User {
            id,
            username,
            password,
            token,
        }
    }
}