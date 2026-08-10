use crate::error::AppError;
use crate::lambda::domain::function::{Function, Runtime};
use crate::lambda::domain::function_version::{FunctionVersion, SourceType};
use crate::lambda::domain::invocation::{Invocation, InvocationStatus, TriggerType};
use crate::platform::{
    FunctionId, FunctionVersionId, InvocationId, OrganizationId, Region, ResourceId, RoleId,
};
use async_trait::async_trait;

#[async_trait]
pub trait FunctionRepository: Send + Sync {
    async fn create(&self, function: &Function) -> Result<(), AppError>;
    async fn find_by_id(&self, id: FunctionId) -> Result<Option<Function>, AppError>;
    async fn find_by_resource_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<Option<Function>, AppError>;
    async fn find_by_name(
        &self,
        organization_id: OrganizationId,
        name: &str,
    ) -> Result<Option<Function>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Function>, AppError>;
    async fn update(&self, function: &Function) -> Result<(), AppError>;
    async fn update_active_version(
        &self,
        id: FunctionId,
        version_id: FunctionVersionId,
    ) -> Result<(), AppError>;
    async fn soft_delete(&self, id: FunctionId) -> Result<(), AppError>;
}

#[async_trait]
pub trait FunctionVersionRepository: Send + Sync {
    async fn create(&self, version: &FunctionVersion) -> Result<(), AppError>;
    async fn find_by_id(&self, id: FunctionVersionId) -> Result<Option<FunctionVersion>, AppError>;
    async fn find_by_function(
        &self,
        function_id: FunctionId,
    ) -> Result<Vec<FunctionVersion>, AppError>;
    async fn find_by_function_and_version(
        &self,
        function_id: FunctionId,
        version: &str,
    ) -> Result<Option<FunctionVersion>, AppError>;
}

#[async_trait]
pub trait InvocationRepository: Send + Sync {
    async fn create(&self, invocation: &Invocation) -> Result<(), AppError>;
    async fn find_by_id(&self, id: InvocationId) -> Result<Option<Invocation>, AppError>;
    async fn find_by_function(&self, function_id: FunctionId) -> Result<Vec<Invocation>, AppError>;
    async fn find_by_function_version(
        &self,
        function_version_id: FunctionVersionId,
    ) -> Result<Vec<Invocation>, AppError>;
    async fn find_by_status(&self, status: InvocationStatus) -> Result<Vec<Invocation>, AppError>;
    async fn update(&self, invocation: &Invocation) -> Result<(), AppError>;
    async fn update_status(
        &self,
        id: InvocationId,
        status: InvocationStatus,
    ) -> Result<(), AppError>;
}
