use crate::platform::OrganizationId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub slug: String,
    pub status: OrganizationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationStatus {
    Active,
    Suspended,
    Deleted,
}
