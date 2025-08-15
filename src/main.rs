use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use axum::http::Method; // <-- Перенес импорт Method наверх для порядка

// --- ЕДИНСТВЕННЫЙ И ПРАВИЛЬНЫЙ БЛОК ИМПОРТОВ TOWER_HTTP ---
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
// --- КОНЕЦ БЛОКА ---

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// --- Подключаем все наши модули (без изменений) ---
mod db;
mod routes;
mod models;
mod handlers;
mod error;
mod state;
mod config;
mod auth;

use state::AppState;
use config::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Настройка логирования (без изменений)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cyb3ria=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    // Подключение к БД (без изменений)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect_db(&database_url).await;
    
    // Создание состояния приложения (без изменений)
    let app_state = AppState { 
        pool,
        config,
     };

    // --- ОБНОВЛЕННЫЙ БЛОК: НАСТРОЙКА CORS ---
    let cors = CorsLayer::new()
        // ИЗМЕНЕНИЕ ЗДЕСЬ: Мы создаем список разрешенных источников
        .allow_origin([
            "http://localhost:8000".parse::<axum::http::HeaderValue>().unwrap(),
            "http://192.168.1.45:8000".parse::<axum::http::HeaderValue>().unwrap(),
            // Можешь добавить и IP другого устройства, с которого заходил, на всякий случай
            "http://192.168.1.96:8000".parse::<axum::http::HeaderValue>().unwrap(), 
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE
        ])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]);
    // --- КОНЕЦ ОБНОВЛЕННОГО БЛОКА ---

    // Создание роутера с состоянием и слоями
    let app = routes::create_routes(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Запуск сервера (без изменений)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}