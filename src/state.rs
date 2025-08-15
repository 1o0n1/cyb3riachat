use sqlx::PgPool;
use crate::config::Config;

// Структура состояния, которую мы будем передавать во все хендлеры.
// `#[derive(Clone)]` необходим, т.к. Axum будет клонировать его для каждого потока.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}