use crate::platform::{ComputeNodeId, Region, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ComputeNode {
    pub id: ComputeNodeId,
    pub resource_id: ResourceId,
    pub region_id: Region,
    pub name: String,
    pub hostname: String,
    pub status: ComputeNodeStatus,
    pub cpu_capacity: u32,
    pub memory_capacity_mb: u32,
    pub cpu_allocated: u32,
    pub memory_allocated_mb: u32,
    pub docker_endpoint: String,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeNodeStatus {
    Active,
    Draining,
    Offline,
    Unhealthy,
}
