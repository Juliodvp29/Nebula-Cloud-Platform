use crate::audit::domain::audit_log::{AuditLog, AuditResult};
use crate::error::AppError;
use crate::platform::{AuditLogId, OrganizationId, ResourceId, UserId};
use async_trait::async_trait;
use chrono::Utc;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn create(&self, log: &AuditLog) -> Result<(), AppError>;
    async fn find_by_id(&self, id: AuditLogId) -> Result<Option<AuditLog>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<AuditLog>, AppError>;
    async fn find_by_actor(&self, actor_user_id: UserId) -> Result<Vec<AuditLog>, AppError>;
    async fn find_by_action(&self, action: &str) -> Result<Vec<AuditLog>, AppError>;
    async fn find_by_resource(&self, resource_id: ResourceId) -> Result<Vec<AuditLog>, AppError>;
    async fn find_by_date_range(
        &self,
        organization_id: OrganizationId,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
    ) -> Result<Vec<AuditLog>, AppError>;
    async fn find_paginated(
        &self,
        organization_id: OrganizationId,
        actor_user_id: Option<UserId>,
        action: Option<&str>,
        resource_id: Option<ResourceId>,
        start: Option<chrono::DateTime<Utc>>,
        end: Option<chrono::DateTime<Utc>>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<AuditLog>, AppError>;
}
