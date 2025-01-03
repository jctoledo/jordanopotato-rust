use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub prompt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Conversation {
    pub user_id: i32,
    pub conversation_summary: Option<String>,
}
