use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub discussion_id: Uuid,
    pub author_id: Uuid, // <- Переименовано с user_id на author_id
    pub content: String,
    pub created_at: DateTime<Utc>,
}