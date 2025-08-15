use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Наш кастомный тип ошибки.
#[derive(Debug)]
pub enum AppError {
    SqlxError(sqlx::Error),
    // НОВЫЙ ВАРИАНТ для ошибок хэширования
    PasswordHashError(argon2::password_hash::Error),
    NotFound,
    Unauthorized,
    InvalidCredentials, // <- ошибка для неудачного входа
    InternalServerError, // <- общая ошибка 500
     
}

// Реализация преобразования ошибки в HTTP-ответ.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(e) => {
                tracing::error!("SQLx error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            // НОВЫЙ БЛОК для обработки ошибки хэширования
            AppError::PasswordHashError(e) => {
                tracing::error!("Password hashing error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password".to_string())
            }
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".to_string())
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Authentication required".to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),

        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Позволяет использовать `?` для `sqlx::Error`.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::SqlxError(err),
        }
    }
}

// НОВАЯ РЕАЛИЗАЦИЯ: Позволяет использовать `?` для `argon2::password_hash::Error`.
impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashError(err)
    }
}