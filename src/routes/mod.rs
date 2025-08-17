use axum::{Router, middleware, routing::get}; // <-- Добавили get
use crate::state::AppState;
use crate::auth::auth_middleware;
use tower_http::services::ServeDir;
use crate::ws; // <-- Импортируем ws модуль

mod users;
mod messages;
mod discussions;
mod comments;

pub fn create_routes(app_state: AppState) -> Router {
     // 1. Защищенные роуты (БЕЗ WebSocket)
    let protected_routes = Router::new()
        .merge(users::create_protected_user_routes())
        .merge(messages::create_message_routes())
        .merge(discussions::create_discussion_routes())
        .merge(comments::create_comment_routes())
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // 2. Публичные роуты
    let public_routes = Router::new()
        .merge(users::create_public_user_routes());

    // 3. Собираем API-роутер. WebSocket-роут добавляется отдельно,
    //    так как у него своя логика аутентификации (через query-параметр).
    let api_router = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .route("/ws", get(ws::ws_handler)); // <-- ВОТ КЛЮЧЕВОЕ ИЗМЕНЕНИЕ
    Router::new()
        .nest("/api", api_router)
        .fallback_service(ServeDir::new("public"))
        .with_state(app_state) // <-- Передаем единое состояние один раз в самом конце
}