use crate::error::AppError;
use crate::jobs::domain::job::{Job, JobStatus};
use crate::jobs::domain::job_attempt::{JobAttempt, JobAttemptStatus};
use crate::platform::{JobAttemptId, JobId, OrganizationId};
use async_trait::async_trait;

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(&self, job: &Job) -> Result<(), AppError>;
    async fn find_by_id(&self, id: JobId) -> Result<Option<Job>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Job>, AppError>;
    async fn find_by_status(&self, status: JobStatus) -> Result<Vec<Job>, AppError>;
    async fn find_ready_jobs(&self, limit: u32) -> Result<Vec<Job>, AppError>;
    async fn update(&self, job: &Job) -> Result<(), AppError>;
    async fn update_status(&self, id: JobId, status: JobStatus) -> Result<(), AppError>;
    async fn increment_attempts(&self, id: JobId) -> Result<(), AppError>;
    async fn mark_started(&self, id: JobId) -> Result<(), AppError>;
    async fn mark_completed(&self, id: JobId) -> Result<(), AppError>;
    async fn mark_failed(&self, id: JobId, error: Option<String>) -> Result<(), AppError>;
}

#[async_trait]
pub trait JobAttemptRepository: Send + Sync {
    async fn create(&self, attempt: &JobAttempt) -> Result<(), AppError>;
    async fn find_by_id(&self, id: JobAttemptId) -> Result<Option<JobAttempt>, AppError>;
    async fn find_by_job(&self, job_id: JobId) -> Result<Vec<JobAttempt>, AppError>;
    async fn update(&self, attempt: &JobAttempt) -> Result<(), AppError>;
    async fn mark_started(&self, id: JobAttemptId, worker_id: String) -> Result<(), AppError>;
    async fn mark_completed(&self, id: JobAttemptId) -> Result<(), AppError>;
    async fn mark_failed(&self, id: JobAttemptId, error: String) -> Result<(), AppError>;
}
