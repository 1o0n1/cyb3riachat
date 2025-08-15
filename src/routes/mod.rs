use axum::{Router, middleware};
use crate::state::AppState;
use crate::auth::auth_middleware;

mod users;
mod messages;
mod discussions;
mod comments;

pub fn create_routes(app_state: AppState) -> Router {
    // Роуты, которые НЕ требуют аутентификации
    let public_routes = Router::new()
        .merge(users::create_public_user_routes());

    // Роуты, которые ТРЕБУЮТ аутентификации
    let protected_routes = Router::new()
        .merge(users::create_protected_user_routes())
        .merge(messages::create_message_routes())
        .merge(discussions::create_discussion_routes())
        .merge(comments::create_comment_routes())
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // Объединяем все в один роутер
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state)
}