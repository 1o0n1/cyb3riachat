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
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize}; 
use crate::auth::Claims; 
use crate::{
    state::AppState,
    models::user::User,
    error::AppError,
};

#[derive(Serialize, sqlx::FromRow)]
pub struct SafeUser {
    pub id: Uuid,
    pub username: String,
    pub public_key: Option<String>,
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<SafeUser>>, AppError> {
    let users = sqlx::query_as::<_, SafeUser>(
        "SELECT id, username, public_key FROM users"
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(users))
}

// ИЗМЕНЕНИЕ: Добавлено поле для зашифрованного ключа
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub public_key: String,
    pub encrypted_private_key: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, AppError> {
    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await.map_err(|_| AppError::InternalServerError)??;
    
    // ИЗМЕНЕНИЕ: Добавляем encrypted_private_key в запрос
    let new_user_id = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, public_key, encrypted_private_key) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        payload.username,
        payload.email,
        password_hash,
        payload.public_key,
        payload.encrypted_private_key
    )
    .fetch_one(&state.pool)
    .await?
    .id;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        new_user_id
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(user))
}

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

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

// ИЗМЕНЕНИЕ: Структура ответа теперь содержит и зашифрованный ключ
#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    encrypted_private_key: Option<String>,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    let password_hash_str = user.password_hash.clone().ok_or(AppError::InvalidCredentials)?;
    let password = payload.password.clone();
    let verification_result = tokio::task::spawn_blocking(move || {
        let parsed_hash = argon2::PasswordHash::new(&password_hash_str)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    }).await.map_err(|_| AppError::InternalServerError)?;

    if verification_result.is_ok() {
        let now = Utc::now();
        let exp = (now + Duration::days(1)).timestamp();
        
        let claims = Claims {
            sub: user.id,
            exp,
            pk: user.public_key.clone().unwrap_or_default(),
        };

        let token = encode(
            &Header::default(), &claims,
            &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
        ).map_err(|_| AppError::InternalServerError)?;

        // ИЗМЕНЕНИЕ: Возвращаем токен и зашифрованный ключ
        Ok(Json(AuthResponse { 
            token,
            encrypted_private_key: user.encrypted_private_key,
        }))
    } else {
        Err(AppError::InvalidCredentials)
    }
}