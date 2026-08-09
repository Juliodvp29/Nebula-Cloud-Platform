use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Configuration(#[from] crate::config::ConfigError),

    #[error("invalid socket address: {0}")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("failed to bind server: {0}")]
    ServerBind(#[from] std::io::Error),
}
