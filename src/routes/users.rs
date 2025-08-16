// /var/www/cyb3ria/src/routes/users.rs

use axum::{Router, routing::{get, post}};
use crate::state::AppState;
use crate::handlers::user_handler;

// --- ВОТ ЭТА ФУНКЦИЯ ПРОПАЛА ---
// Роуты, не требующие аутентификации (регистрация и вход)
pub fn create_public_user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/login", post(user_handler::login))
}
// --- КОНЕЦ ВОССТАНОВЛЕННОГО БЛОКА ---

// Роуты, требующие аутентификации (получение инфо о юзере и списка всех юзеров)
pub fn create_protected_user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user_handler::get_all_users)) // <- Получение списка
        .route("/users/{id}", get(user_handler::get_user))   // <- Получение одного по ID
}