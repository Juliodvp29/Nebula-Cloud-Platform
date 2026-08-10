use crate::auth::domain::repositories::SessionRepository;
use crate::auth::domain::session::SessionId;
use crate::error::AppError;
use crate::platform::UserId;

pub struct RevokeSessionInput {
    pub session_id: SessionId,
    pub user_id: UserId,
}

pub async fn revoke_session<S: SessionRepository>(
    session_repo: &S,
    input: RevokeSessionInput,
) -> Result<(), AppError> {
    let session = session_repo
        .find_by_id(input.session_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if session.user_id != input.user_id {
        return Err(AppError::Validation("unauthorized".to_string()));
    }

    session_repo.revoke(input.session_id).await?;
    Ok(())
}
