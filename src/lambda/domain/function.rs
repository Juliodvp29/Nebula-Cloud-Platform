use crate::platform::{
    FunctionId, FunctionVersionId, OrganizationId, Region, ResourceId, RoleId, UserId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Function {
    pub id: FunctionId,
    pub resource_id: ResourceId,
    pub organization_id: OrganizationId,
    pub region_id: Region,
    pub name: String,
    pub description: Option<String>,
    pub runtime: Runtime,
    pub handler: String,
    pub timeout_ms: u32,
    pub memory_mb: u32,
    pub execution_role_id: RoleId,
    pub active_version_id: Option<FunctionVersionId>,
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    NodeJS,
    Python,
    Custom,
}
