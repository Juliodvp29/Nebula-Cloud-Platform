use crate::auth::domain::access_key::{AccessKey, AccessKeyId, AccessKeyStatus};
use crate::auth::domain::password::PasswordError;
use crate::auth::domain::repositories::AccessKeyRepository;
use crate::error::AppError;
use crate::platform::{OrganizationId, UserId};
use chrono::Utc;
use uuid::Uuid;

pub struct CreateAccessKeyInput {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub description: Option<String>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

pub struct CreateAccessKeyOutput {
    pub access_key: AccessKey,
    pub secret: String, // Only returned once!
}

pub async fn create_access_key<R: AccessKeyRepository>(
    repo: &R,
    input: CreateAccessKeyInput,
) -> Result<CreateAccessKeyOutput, AppError> {
    let access_key_id = format!(
        "mcloud-{}",
        Uuid::new_v4().to_string().replace('-', "").to_lowercase()
    );
    let secret = generate_secret();
    let secret_hash = hash_secret(&secret).map_err(|_| AppError::Internal)?;

    let now = Utc::now();
    let access_key = AccessKey {
        id: AccessKeyId::new(),
        user_id: input.user_id,
        organization_id: input.organization_id,
        access_key_id: access_key_id.clone(),
        secret_hash,
        description: input.description,
        status: AccessKeyStatus::Active,
        last_used_at: None,
        expires_at: input.expires_at,
        created_at: now,
        revoked_at: None,
    };

    repo.create(&access_key).await?;

    Ok(CreateAccessKeyOutput { access_key, secret })
}

fn generate_secret() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let secret: String = (0..40)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect();
    format!("mcloud-{}", secret)
}

fn hash_secret(secret: &str) -> Result<String, PasswordError> {
    use argon2::password_hash::{SaltString, rand_core::OsRng};
    use argon2::{Argon2, PasswordHasher};
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(secret.as_bytes(), &salt)
        .map_err(|e| PasswordError::PasswordHash(e.to_string()))
        .map(|h| h.to_string())
}
