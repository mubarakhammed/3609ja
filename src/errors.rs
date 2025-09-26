use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

/// Application result type
pub type AppResult<T> = Result<T, AppError>;

/// Main application error type
#[derive(Error, Debug, ToSchema)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Coordinates error: {0}")]
    Coordinates(#[from] crate::domain::value_objects::CoordinatesError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Serialization(_) => (StatusCode::BAD_REQUEST, "Invalid data format"),
            AppError::Coordinates(_) => (StatusCode::BAD_REQUEST, "Invalid coordinates"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
