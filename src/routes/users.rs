use axum::{Router, routing::{get, post}};
use crate::state::AppState;
use crate::handlers::user_handler;

// Роуты, не требующие аутентификации
pub fn create_public_user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/login", post(user_handler::login))
}

// Роуты, требующие аутентификации
pub fn create_protected_user_routes() -> Router<AppState> {
    Router::new()
        .route("/users/{id}", get(user_handler::get_user))
}