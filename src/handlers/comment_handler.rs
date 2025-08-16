use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use crate::{
    state::AppState,
    models::comment::Comment,
    error::AppError,
};
use axum::Extension;

#[derive(serde::Deserialize)]
pub struct CreateCommentPayload {
    pub discussion_id: Uuid,
    pub content: String,
}

pub async fn create_comment(
    Extension(user_id): Extension<Uuid>, // <-- ПОЛУЧАЕМ ID автора из токена
    State(state): State<AppState>,
    Json(payload): Json<CreateCommentPayload>,
) -> Result<Json<Comment>, AppError> {
    let comment = sqlx::query_as!(
        Comment,
        "INSERT INTO comments (discussion_id, author_id, content) VALUES ($1, $2, $3) RETURNING *",
        payload.discussion_id,
        user_id, // <-- ИСПОЛЬЗУЕМ безопасный ID из токена
        payload.content
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(comment))
}

pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Comment>, AppError> {
    let comment = sqlx::query_as!(
        Comment,
        "SELECT * FROM comments WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(comment))
}