use crate::compute::domain::compute_node::{ComputeNode, ComputeNodeStatus};
use crate::compute::domain::instance::{Instance, InstanceStatus};
use crate::compute::domain::instance_type::{InstanceType, InstanceTypeStatus};
use crate::error::AppError;
use crate::platform::{
    ComputeNodeId, InstanceId, InstanceTypeId, OrganizationId, Region, ResourceId,
};
use async_trait::async_trait;

#[async_trait]
pub trait InstanceRepository: Send + Sync {
    async fn create(&self, instance: &Instance) -> Result<(), AppError>;
    async fn find_by_id(&self, id: InstanceId) -> Result<Option<Instance>, AppError>;
    async fn find_by_resource_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<Option<Instance>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Instance>, AppError>;
    async fn find_by_compute_node(
        &self,
        compute_node_id: ComputeNodeId,
    ) -> Result<Vec<Instance>, AppError>;
    async fn find_by_status(&self, status: InstanceStatus) -> Result<Vec<Instance>, AppError>;
    async fn update(&self, instance: &Instance) -> Result<(), AppError>;
    async fn update_status(&self, id: InstanceId, status: InstanceStatus) -> Result<(), AppError>;
    async fn set_docker_container_id(
        &self,
        id: InstanceId,
        container_id: String,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait InstanceTypeRepository: Send + Sync {
    async fn create(&self, instance_type: &InstanceType) -> Result<(), AppError>;
    async fn find_by_id(&self, id: InstanceTypeId) -> Result<Option<InstanceType>, AppError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<InstanceType>, AppError>;
    async fn list_active(&self) -> Result<Vec<InstanceType>, AppError>;
    async fn update(&self, instance_type: &InstanceType) -> Result<(), AppError>;
    async fn update_status(
        &self,
        id: InstanceTypeId,
        status: InstanceTypeStatus,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait ComputeNodeRepository: Send + Sync {
    async fn create(&self, node: &ComputeNode) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ComputeNodeId) -> Result<Option<ComputeNode>, AppError>;
    async fn find_by_resource_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<Option<ComputeNode>, AppError>;
    async fn find_by_region(&self, region: Region) -> Result<Vec<ComputeNode>, AppError>;
    async fn find_active_by_region(&self, region: Region) -> Result<Vec<ComputeNode>, AppError>;
    async fn update(&self, node: &ComputeNode) -> Result<(), AppError>;
    async fn update_status(
        &self,
        id: ComputeNodeId,
        status: ComputeNodeStatus,
    ) -> Result<(), AppError>;
    async fn update_heartbeat(&self, id: ComputeNodeId) -> Result<(), AppError>;
    async fn update_allocated_resources(
        &self,
        id: ComputeNodeId,
        cpu: u32,
        memory_mb: u32,
    ) -> Result<(), AppError>;
}
