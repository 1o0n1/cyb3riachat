use axum::{
    extract::{Path, State, Json},
    Extension,
};
use uuid::Uuid;
use crate::{
    state::AppState, // <-- ИЗМЕНЕНИЕ: Используем единый AppState
    models::message::Message,
    error::AppError,
};

#[derive(serde::Deserialize)]
pub struct CreateMessagePayload {
    pub recipient_id: Uuid,
    pub content: String,
}

// ИЗМЕНЕНИЕ: Убираем второй State, теперь все в одном AppState
pub async fn create_message(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CreateMessagePayload>,
) -> Result<Json<Message>, AppError> {
    if user_id == payload.recipient_id {
        return Err(AppError::InternalServerError); 
    }

    let message = sqlx::query_as!(
        Message,
        "INSERT INTO messages (user_id, recipient_id, content) VALUES ($1, $2, $3) RETURNING *",
        user_id,
        payload.recipient_id,
        payload.content
    )
    .fetch_one(&state.pool)
    .await?;

    // ИЗМЕНЕНИЕ: Обращаемся к ws_state через общее состояние state
    if let Err(e) = state.ws_state.tx.send((message.clone(), user_id)) {
        tracing::error!("Failed to broadcast message: {}", e);
    }
    
    Ok(Json(message))
}

pub async fn get_conversation_with(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Path(partner_id): Path<Uuid>,
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = sqlx::query_as!(
        Message,
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