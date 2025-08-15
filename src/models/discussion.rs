use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discussion {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid, // <- Добавлено это поле
    pub created_at: DateTime<Utc>,
}