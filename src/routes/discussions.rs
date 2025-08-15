use axum::{Router, routing::{get, post}};
use crate::state::AppState;
use crate::handlers::discussion_handler; // <- Указываем на новый хендлер

pub fn create_discussion_routes() -> Router<AppState> {
    Router::new()
        .route("/discussions", post(discussion_handler::create_discussion))
        .route("/discussions/{id}", get(discussion_handler::get_discussion))
}