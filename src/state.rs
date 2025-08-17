use sqlx::PgPool;
use crate::config::Config;
use crate::ws::WsState; // Импортируем состояние WebSocket

// ИЗМЕНЕНИЕ: Создаем единую структуру состояния для всего приложения.
// `#[derive(FromRef)]` позволяет хендлерам извлекать части состояния,
// например `State<Config>` или `State<WsState>`, из общего `State<AppState>`.
#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub ws_state: WsState, // Включаем состояние WebSocket сюда
}