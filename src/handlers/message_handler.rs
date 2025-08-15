// /var/www/cyb3ria/src/handlers/message_handler.rs

use axum::{
    extract::{Path, State, Json},
    Extension, // <-- Новый экстрактор!
};
// ... другие импорты
use uuid::Uuid;
use crate::{
    state::AppState,
    models::message::Message,
    error::AppError,
};
// ...

#[derive(serde::Deserialize)]
pub struct CreateMessagePayload {
    pub content: String,
}

#[axum::debug_handler]
pub async fn create_message(
    Extension(user_id): Extension<Uuid>, // <-- ВОТ ТАК мы получаем ID!
    State(state): State<AppState>,
    Json(payload): Json<CreateMessagePayload>,
) -> Result<Json<Message>, AppError> {
    let message = sqlx::query_as!(
        Message,
        "INSERT INTO messages (user_id, content) VALUES ($1, $2) RETURNING *",
        user_id, // <-- Используем ID из расширений
        payload.content
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(message))
}

// ... get_message остается без изменений ...
pub async fn get_message(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Message>, AppError> {
    let message = sqlx::query_as!(
        Message,
        "SELECT * FROM messages WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(message))
}