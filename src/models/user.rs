use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub public_key: Option<String>,

    // Добавляем недостающее поле
    // Оно Option<String>, так как в БД колонка может быть NULL.
    // Оно скрыто из JSON-ответов для безопасности.
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}