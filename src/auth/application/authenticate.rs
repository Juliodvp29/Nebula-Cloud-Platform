use crate::auth::domain::jwt::Claims;
use crate::auth::domain::password::verify_password;
use crate::auth::domain::repositories::{SessionRepository, UserRepository};
use crate::auth::domain::session::{Session, SessionId};
use crate::auth::domain::user::User;
use crate::error::AppError;
use crate::platform::{OrganizationId, UserId};
use chrono::Utc;

pub struct AuthenticateInput {
    pub email: String,
    pub password: String,
    pub organization_id: OrganizationId,
}

pub struct AuthenticateOutput {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_claims: Claims,
}

pub async fn authenticate<R: UserRepository, S: SessionRepository>(
    user_repo: &R,
    session_repo: &S,
    jwt_config: &crate::auth::domain::jwt::JwtConfig,
    input: AuthenticateInput,
) -> Result<AuthenticateOutput, AppError> {
    let user = user_repo
        .find_by_email(&input.email)
        .await?
        .ok_or(AppError::Validation("invalid credentials".to_string()))?;

    verify_password(&input.password, &user.password_hash)?;

    let (access_token, access_token_claims) = Claims::new(
        user.id,
        input.organization_id,
        jwt_config.access_token_ttl,
        &jwt_config.secret,
    )?;

    let refresh_session = Session {
        id: SessionId::new(),
        user_id: user.id,
        organization_id: input.organization_id,
        token_hash: hash_token(&access_token),
        expires_at: Utc::now() + jwt_config.refresh_token_ttl,
        revoked_at: None,
        created_at: Utc::now(),
        last_used_at: Utc::now(),
    };

    session_repo.create(&refresh_session).await?;
    user_repo.update_last_login(user.id).await?;

    Ok(AuthenticateOutput {
        user,
        access_token: access_token.clone(),
        refresh_token: access_token,
        access_token_claims,
    })
}

fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
