use axum::{Router, routing::{get, post}};
use crate::state::AppState;
use crate::handlers::comment_handler; // <- Указываем на новый хендлер

pub fn create_comment_routes() -> Router<AppState> {
    Router::new()
        .route("/comments", post(comment_handler::create_comment))
        .route("/comments/{id}", get(comment_handler::get_comment))
}