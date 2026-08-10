use crate::platform::{OrganizationId, UserId};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("token expired")]
    Expired,
    #[error("invalid token")]
    InvalidToken,
    #[error("missing secret")]
    MissingSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

impl Claims {
    pub fn new(
        user_id: UserId,
        organization_id: OrganizationId,
        ttl: Duration,
        secret: &str,
    ) -> Result<(String, Self), JwtError> {
        let now = Utc::now();
        let jti = uuid::Uuid::new_v4().to_string();
        let exp = (now + ttl).timestamp();
        let iat = now.timestamp();

        let claims = Self {
            sub: user_id.to_string(),
            user_id,
            organization_id,
            iat,
            exp,
            jti,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok((token, claims))
    }

    pub fn validate(token: &str, secret: &str) -> Result<Self, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}

pub struct JwtConfig {
    pub secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, JwtError> {
        let secret = std::env::var("NEBULA_JWT_SECRET").map_err(|_| JwtError::MissingSecret)?;

        let access_token_ttl = std::env::var("NEBULA_ACCESS_TOKEN_TTL")
            .unwrap_or_else(|_| "15m".to_string())
            .parse::<humantime::Duration>()
            .map(|d| Duration::from_std(d.into()).unwrap_or(Duration::minutes(15)))
            .unwrap_or(Duration::minutes(15));

        let refresh_token_ttl = std::env::var("NEBULA_REFRESH_TOKEN_TTL")
            .unwrap_or_else(|_| "7d".to_string())
            .parse::<humantime::Duration>()
            .map(|d| Duration::from_std(d.into()).unwrap_or(Duration::days(7)))
            .unwrap_or(Duration::days(7));

        Ok(Self {
            secret,
            access_token_ttl,
            refresh_token_ttl,
        })
    }
}
