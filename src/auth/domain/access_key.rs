use crate::platform::{OrganizationId, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AccessKey {
    pub id: AccessKeyId,
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub access_key_id: String,
    pub secret_hash: String,
    pub description: Option<String>,
    pub status: AccessKeyStatus,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKeyStatus {
    Active,
    Inactive,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccessKeyId(pub uuid::Uuid);

impl AccessKeyId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for AccessKeyId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AccessKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<uuid::Uuid> for AccessKeyId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl From<AccessKeyId> for uuid::Uuid {
    fn from(id: AccessKeyId) -> Self {
        id.0
    }
}
