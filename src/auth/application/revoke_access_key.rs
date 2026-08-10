use crate::auth::domain::access_key::AccessKeyId;
use crate::auth::domain::repositories::AccessKeyRepository;
use crate::error::AppError;
use crate::platform::UserId;

pub struct RevokeAccessKeyInput {
    pub access_key_id: AccessKeyId,
    pub user_id: UserId,
}

pub async fn revoke_access_key<R: AccessKeyRepository>(
    repo: &R,
    input: RevokeAccessKeyInput,
) -> Result<(), AppError> {
    let key = repo
        .find_by_id(input.access_key_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if key.user_id != input.user_id {
        return Err(AppError::Validation("unauthorized".to_string()));
    }

    repo.revoke(input.access_key_id).await?;
    Ok(())
}
