use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ИЗМЕНЕНИЕ: Добавили Clone, чтобы сообщения можно было рассылать через broadcast
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipient_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}