use async_trait::async_trait;

use crate::error::AppError;

use super::{
    organization_id::OrganizationId, resource_id::ResourceId,
    resource_identifier::ResourceIdentifier, resource_type::ResourceType,
};

#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn create(
        &self,
        id: ResourceId,
        organization_id: OrganizationId,
        resource_type: ResourceType,
        resource_identifier: ResourceIdentifier,
    ) -> Result<(), AppError>;

    async fn find_by_id(&self, id: ResourceId) -> Result<bool, AppError>;

    async fn find_by_identifier(&self, identifier: &ResourceIdentifier) -> Result<bool, AppError>;
}
