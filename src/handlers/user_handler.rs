// /var/www/cyb3ria/src/handlers/user_handler.rs

// --- БЛОК 1: Импорты ---
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordVerifier,
        PasswordHasher, 
        SaltString
    },
    Argon2
};
// Новые импорты для JWT
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize}; // Добавили Deserialize для Claims
use crate::auth::Claims; 

use crate::{
    state::AppState,
    models::user::User,
    error::AppError,
};


// --- БЛОК 2: Структура для создания пользователя ---
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

// --- БЛОК 3: Функция создания пользователя (остается без изменений) ---
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, AppError> {
    // ... ваш работающий код для create_user ...
    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await
    .map_err(|e| {
        tracing::error!("JoinError during password hashing: {:?}", e);
        AppError::PasswordHashError(argon2::password_hash::Error::PhcStringField)
    })?
    ?;
    
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        payload.username,
        payload.email,
        password_hash
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}

// --- БЛОК 4: Функция получения пользователя (остается без изменений) ---
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}

// --- БЛОК 5: Структура для данных входа (остается без изменений) ---
#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

// Структура для ответа API после успешного входа
#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

// --- БЛОК 7: ОБНОВЛЕННАЯ функция login ---
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, AppError> { // Обратите внимание на тип возврата!
    // 1. Найти пользователя по email
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    // 2. Проверить и склонировать хэш пароля
    let password_hash = user.password_hash.clone().ok_or(AppError::InvalidCredentials)?;

    // 3. Сравнить пароль с хэшем
    let password = payload.password;
    let verification_result = tokio::task::spawn_blocking(move || {
        let parsed_hash = match argon2::PasswordHash::new(&password_hash) {
            Ok(hash) => hash,
            Err(e) => return Err(e),
        };
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    })
    .await
    .map_err(|e| {
        tracing::error!("JoinError during password verification: {:?}", e);
        AppError::InternalServerError
    })?;

    // Если пароль верный, генерируем токен
    if verification_result.is_ok() {
        // --- Начало блока генерации JWT ---
        let now = Utc::now();
        // let iat = now.timestamp(); // iat (issued at) обычно не нужен, т.к. есть exp
        let exp = (now + Duration::days(1)).timestamp(); // Токен живет 1 дней
        
        let claims = Claims {
            sub: user.id,
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("JWT encoding error: {:?}", e);
            AppError::InternalServerError
        })?;
        // --- Конец блока генерации JWT ---

        // Возвращаем токен
        Ok(Json(AuthResponse { token }))
    } else {
        // Если пароль неверный
        tracing::debug!("Invalid password attempt for email: {}", payload.email);
        Err(AppError::InvalidCredentials)
    }
}