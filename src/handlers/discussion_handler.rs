use axum::{
    extract::{Path, State},
    Json, 
};
use axum::Extension;

use uuid::Uuid;
use crate::{
    state::AppState,
    models::discussion::Discussion,
    error::AppError,
};


#[derive(serde::Deserialize)]
pub struct CreateDiscussionPayload {
    pub title: String,
}

pub async fn create_discussion(
    Extension(user_id): Extension<Uuid>, // <-- Получаем ID автора из токена
    State(state): State<AppState>,
    Json(payload): Json<CreateDiscussionPayload>,
) -> Result<Json<Discussion>, AppError> {
    let discussion = sqlx::query_as!(
        Discussion,
        "INSERT INTO discussions (title, author_id) VALUES ($1, $2) RETURNING *",
        payload.title,
        user_id // <-- Используем проверенный ID
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(discussion))
}

pub async fn get_discussion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Discussion>, AppError> {
    let discussion = sqlx::query_as!(
        Discussion,
        "SELECT * FROM discussions WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(discussion))
}