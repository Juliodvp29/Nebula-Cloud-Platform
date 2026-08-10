use crate::platform::{JobId, OrganizationId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub organization_id: OrganizationId,
    pub job_type: String,
    pub status: JobStatus,
    pub priority: i32,
    pub payload: serde_json::Value,
    pub attempts: u32,
    pub max_attempts: u32,
    pub available_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    DeadLetter,
}
