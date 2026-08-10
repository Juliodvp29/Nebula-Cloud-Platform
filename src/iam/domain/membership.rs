use crate::platform::{OrganizationId, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Membership {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub status: MembershipStatus,
    pub joined_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipStatus {
    Active,
    Pending,
    Removed,
}
