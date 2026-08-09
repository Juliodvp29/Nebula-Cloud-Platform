use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("NEBULA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("NEBULA_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort)?;

        Ok(Self { host, port })
    }
}

#[derive(Debug, thiserror::Error)]

pub enum ConfigError {
    #[error("NEBULA_PORT must be a valid port number")]
    InvalidPort,
}
