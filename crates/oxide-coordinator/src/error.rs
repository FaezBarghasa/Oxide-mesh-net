//! Coordinator error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoordinatorError {
    #[error("MQTT error: {0}")]
    Mqtt(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authz(String),

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("OIDC error: {0}")]
    Oidc(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Actix error: {0}")]
    Actix(#[from] actix_web::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl actix_web::ResponseError for CoordinatorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            CoordinatorError::Auth(_) | CoordinatorError::Jwt(_) => {
                actix_web::http::StatusCode::UNAUTHORIZED
            }
            CoordinatorError::Authz(_) => actix_web::http::StatusCode::FORBIDDEN,
            CoordinatorError::NodeNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            CoordinatorError::Config(_) | CoordinatorError::Serialization(_) => {
                actix_web::http::StatusCode::BAD_REQUEST
            }
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type Result<T> = std::result::Result<T, CoordinatorError>;
