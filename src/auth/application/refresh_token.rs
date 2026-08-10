use crate::auth::domain::jwt::{Claims, JwtConfig};
use crate::auth::domain::repositories::SessionRepository;
use crate::auth::domain::session::{Session, SessionId};
use crate::error::AppError;
use crate::platform::{OrganizationId, UserId};
use chrono::Utc;

pub struct RefreshTokenInput {
    pub refresh_token: String,
    pub organization_id: OrganizationId,
}

pub struct RefreshTokenOutput {
    pub access_token: String,
    pub access_token_claims: Claims,
}

pub async fn refresh_token<S: SessionRepository>(
    session_repo: &S,
    jwt_config: &JwtConfig,
    input: RefreshTokenInput,
) -> Result<RefreshTokenOutput, AppError> {
    let token_hash = hash_token(&input.refresh_token);

    let session = session_repo
        .find_by_token_hash(&token_hash)
        .await?
        .ok_or(AppError::Validation("invalid refresh token".to_string()))?;

    if session.revoked_at.is_some() {
        return Err(AppError::Validation("session revoked".to_string()));
    }
    if session.expires_at < Utc::now() {
        return Err(AppError::Validation("refresh token expired".to_string()));
    }

    let (access_token, access_token_claims) = Claims::new(
        session.user_id,
        session.organization_id,
        jwt_config.access_token_ttl,
        &jwt_config.secret,
    )?;

    session_repo.update_last_used(session.id).await?;

    Ok(RefreshTokenOutput {
        access_token,
        access_token_claims,
    })
}

fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
