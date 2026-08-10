use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("resource already exists: {0}")]
    Conflict(String),

    #[error("resource not found")]
    NotFound,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("configuration error: {0}")]
    Configuration(#[from] crate::config::ConfigError),

    #[error("address parsing error: {0}")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("password error: {0}")]
    Password(#[from] crate::auth::domain::password::PasswordError),

    #[error("jwt error: {0}")]
    Jwt(#[from] crate::auth::domain::jwt::JwtError),

    #[error("internal error")]
    Internal,
}
