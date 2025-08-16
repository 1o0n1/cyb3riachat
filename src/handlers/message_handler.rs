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
    pub recipient_id: Uuid,
    pub content: String, // Сюда придет зашифрованная строка в Base64
}

#[axum::debug_handler]
pub async fn create_message(
    Extension(user_id): Extension<Uuid>, // <-- ВОТ ТАК мы получаем ID!
    State(state): State<AppState>,
    Json(payload): Json<CreateMessagePayload>,
) -> Result<Json<Message>, AppError> {
    // Проверяем, что пользователь не отправляет сообщение сам себе
    if user_id == payload.recipient_id {
        // Здесь можно вернуть кастомную ошибку, но для простоты пока оставим так
        return Err(AppError::InternalServerError); 
    }

    let message = sqlx::query_as!(
        Message,
        "INSERT INTO messages (user_id, recipient_id, content) VALUES ($1, $2, $3) RETURNING *",
        user_id, // Отправитель
        payload.recipient_id, // Получатель
        payload.content // Зашифрованное сообщение
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

// --- НОВАЯ ФУНКЦИЯ ---
pub async fn get_conversation_with(
    Extension(user_id): Extension<Uuid>, // ID текущего пользователя
    State(state): State<AppState>,
    Path(partner_id): Path<Uuid>, // ID собеседника из URL
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = sqlx::query_as!(
        Message,
        // Этот запрос выбирает все сообщения, где текущий юзер - отправитель, а собеседник - получатель,
        // ИЛИ где собеседник - отправитель, а текущий юзер - получатель.
        // И сортирует их по дате создания.
        "SELECT * FROM messages 
         WHERE (user_id = $1 AND recipient_id = $2) OR (user_id = $2 AND recipient_id = $1)
         ORDER BY created_at ASC",
        user_id,
        partner_id
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}