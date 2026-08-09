use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::error::AppError;
use crate::platform::{
    OrganizationId, ResourceId, ResourceIdentifier, ResourceRepository, ResourceType,
};

#[derive(Debug, Clone)]
struct ResourceRecord {
    id: ResourceId,
    organization_id: OrganizationId,
    resource_type: ResourceType,
    resource_identifier: ResourceIdentifier,
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryResourceRepository {
    resources: Arc<RwLock<HashMap<ResourceId, ResourceRecord>>>,
}

impl InMemoryResourceRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ResourceRepository for InMemoryResourceRepository {
    async fn create(
        &self,
        id: ResourceId,
        organization_id: OrganizationId,
        resource_type: ResourceType,
        resource_identifier: ResourceIdentifier,
    ) -> Result<(), AppError> {
        let mut resources = self.resources.write().await;

        if resources.contains_key(&id) {
            return Err(AppError::Conflict(format!(
                "resource '{}' already exists",
                id.as_uuid()
            )));
        }

        if resources
            .values()
            .any(|resource| resource.resource_identifier == resource_identifier)
        {
            return Err(AppError::Conflict(format!(
                "resource '{}' already exists",
                resource_identifier
            )));
        }

        resources.insert(
            id,
            ResourceRecord {
                id,
                organization_id,
                resource_type,
                resource_identifier,
            },
        );

        Ok(())
    }

    async fn find_by_id(&self, id: ResourceId) -> Result<bool, AppError> {
        let resources = self.resources.read().await;

        Ok(resources.contains_key(&id))
    }

    async fn find_by_identifier(&self, identifier: &ResourceIdentifier) -> Result<bool, AppError> {
        let resources = self.resources.read().await;

        Ok(resources
            .values()
            .any(|resource| &resource.resource_identifier == identifier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_repository() -> InMemoryResourceRepository {
        InMemoryResourceRepository::new()
    }

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    #[tokio::test]
    async fn creates_resource_successfully() {
        let repository = create_repository();

        let resource_id = ResourceId::new();
        let organization_id = organization_id();
        let cloud_id = crate::platform::CloudId::new("bucket-prod-001").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::S3Bucket, &cloud_id);

        repository
            .create(
                resource_id,
                organization_id,
                ResourceType::S3Bucket,
                identifier.clone(),
            )
            .await
            .unwrap();

        assert!(repository.find_by_id(resource_id).await.unwrap());

        assert!(repository.find_by_identifier(&identifier).await.unwrap());
    }

    #[tokio::test]
    async fn rejects_duplicate_resource_id() {
        let repository = create_repository();

        let resource_id = ResourceId::new();
        let organization_id = organization_id();

        let cloud_id = crate::platform::CloudId::new("bucket-prod-001").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::S3Bucket, &cloud_id);

        repository
            .create(
                resource_id,
                organization_id,
                ResourceType::S3Bucket,
                identifier.clone(),
            )
            .await
            .unwrap();

        let result = repository
            .create(
                resource_id,
                organization_id,
                ResourceType::S3Bucket,
                identifier,
            )
            .await;

        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn rejects_duplicate_resource_identifier() {
        let repository = create_repository();

        let organization_id = organization_id();

        let cloud_id = crate::platform::CloudId::new("bucket-prod-001").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::S3Bucket, &cloud_id);

        repository
            .create(
                ResourceId::new(),
                organization_id,
                ResourceType::S3Bucket,
                identifier.clone(),
            )
            .await
            .unwrap();

        let result = repository
            .create(
                ResourceId::new(),
                organization_id,
                ResourceType::S3Bucket,
                identifier,
            )
            .await;

        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn returns_false_for_unknown_resource() {
        let repository = create_repository();

        let resource_id = ResourceId::new();

        assert!(!repository.find_by_id(resource_id).await.unwrap());
    }
}
