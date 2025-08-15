use axum::{Router, routing::{get, post}};
use crate::state::AppState;
use crate::handlers::message_handler; // <- Указываем на новый хендлер

pub fn create_message_routes() -> Router<AppState> {
    Router::new()
        .route("/messages", post(message_handler::create_message))
        .route("/messages/{id}", get(message_handler::get_message))
}