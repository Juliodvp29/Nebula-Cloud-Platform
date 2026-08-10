use crate::platform::{
    ComputeNodeId, InstanceId, InstanceTypeId, OrganizationId, Region, ResourceId, UserId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: InstanceId,
    pub resource_id: ResourceId,
    pub organization_id: OrganizationId,
    pub region_id: Region,
    pub compute_node_id: Option<ComputeNodeId>,
    pub instance_type_id: InstanceTypeId,
    pub name: String,
    pub image: String,
    pub docker_container_id: Option<String>,
    pub status: InstanceStatus,
    pub cpu_limit: Option<u32>,
    pub memory_limit_mb: Option<u32>,
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub terminated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceStatus {
    Pending,
    Running,
    Stopping,
    Stopped,
    Starting,
    Restarting,
    Terminating,
    Terminated,
    Failed,
}
