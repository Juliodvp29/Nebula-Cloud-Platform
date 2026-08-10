use crate::platform::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub status: UserStatus,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Suspended,
    Deleted,
}
