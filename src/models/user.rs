use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub public_key: Option<String>,

    // НОВОЕ ПОЛЕ для зашифрованного ключа
    #[serde(skip_serializing)] // Не отправляем его в JSON по умолчанию
    pub encrypted_private_key: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}