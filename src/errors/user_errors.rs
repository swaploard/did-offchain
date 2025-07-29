use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Internal Server Error")]
    InternalError,
    #[error("Invalid Credentials")]
    BadCredentials,
    #[error("User Already Exists")]
    UserExists,
    #[error("Database Error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Hashing Error")]
    HashError,
    #[error("JWT Creation Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            UserServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            UserServiceError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::BadCredentials => StatusCode::UNAUTHORIZED,
            UserServiceError::UserExists => StatusCode::CONFLICT,
            UserServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UserServiceError::DbError(_)
            | UserServiceError::HashError
            | UserServiceError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let err_json = ErrorResponse {
            error: self.to_string(),
        };

        HttpResponse::build(status).json(err_json)
    }
}
