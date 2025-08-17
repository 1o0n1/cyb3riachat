use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use axum::http::Method;

use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod routes;
mod models;
mod handlers;
mod error;
mod state;
mod config;
mod auth;
mod ws;

use state::AppState;
use config::Config;
use ws::WsState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cyb3ria=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect_db(&database_url).await;
    
    // ИЗМЕНЕНИЕ: Создаем единое состояние приложения
    let app_state = AppState { 
        pool,
        config,
        ws_state: WsState::new(), // Инициализируем WsState здесь
     };

    let cors = CorsLayer::new()
        .allow_origin([
            "https://cyb3ria.xyz".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
            "http://192.168.1.45:3000".parse().unwrap(),
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE
        ])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]);

    // ИЗМЕНЕНИЕ: Создаем роутер с единым состоянием
    let app = routes::create_routes(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}