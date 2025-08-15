use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


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

    // Настройка логирования
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cyb3ria=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    // Подключение к БД
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect_db(&database_url).await;
    
    // Создание состояния приложения
    let app_state = AppState { 
        pool,
        config,
     };

    // Создание роутера с состоянием
    let app = routes::create_routes(app_state)
        .layer(TraceLayer::new_for_http());

    // Запуск сервера
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}