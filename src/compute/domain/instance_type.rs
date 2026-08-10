use crate::platform::{InstanceTypeId, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct InstanceType {
    pub id: InstanceTypeId,
    pub resource_id: ResourceId,
    pub name: String,
    pub cpu: u32,
    pub memory_mb: u32,
    pub disk_gb: u32,
    pub description: Option<String>,
    pub status: InstanceTypeStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceTypeStatus {
    Active,
    Inactive,
}
