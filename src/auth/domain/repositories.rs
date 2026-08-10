use crate::auth::domain::access_key::{AccessKey, AccessKeyId};
use crate::auth::domain::session::{Session, SessionId};
use crate::auth::domain::user::{User, UserStatus};
use crate::error::AppError;
use crate::platform::{OrganizationId, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn update(&self, user: &User) -> Result<(), AppError>;
    async fn update_status(&self, id: UserId, status: UserStatus) -> Result<(), AppError>;
    async fn update_last_login(&self, id: UserId) -> Result<(), AppError>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &Session) -> Result<(), AppError>;
    async fn find_by_id(&self, id: SessionId) -> Result<Option<Session>, AppError>;
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<Session>, AppError>;
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Session>, AppError>;
    async fn revoke(&self, id: SessionId) -> Result<(), AppError>;
    async fn revoke_all_for_user(&self, user_id: UserId) -> Result<(), AppError>;
    async fn update_last_used(&self, id: SessionId) -> Result<(), AppError>;
    async fn delete_expired(&self) -> Result<u64, AppError>;
}

#[async_trait]
pub trait AccessKeyRepository: Send + Sync {
    async fn create(&self, key: &AccessKey) -> Result<(), AppError>;
    async fn find_by_id(&self, id: AccessKeyId) -> Result<Option<AccessKey>, AppError>;
    async fn find_by_access_key_id(
        &self,
        access_key_id: &str,
    ) -> Result<Option<AccessKey>, AppError>;
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<AccessKey>, AppError>;
    async fn update(&self, key: &AccessKey) -> Result<(), AppError>;
    async fn revoke(&self, id: AccessKeyId) -> Result<(), AppError>;
    async fn update_last_used(&self, id: AccessKeyId) -> Result<(), AppError>;
}
