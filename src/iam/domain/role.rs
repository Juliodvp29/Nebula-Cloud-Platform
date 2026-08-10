use crate::platform::{OrganizationId, RoleId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub organization_id: OrganizationId,
    pub name: String,
    pub description: Option<String>,
    pub is_system_role: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
