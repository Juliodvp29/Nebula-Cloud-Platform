use crate::platform::{JobAttemptId, JobId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct JobAttempt {
    pub id: JobAttemptId,
    pub job_id: JobId,
    pub attempt_number: u32,
    pub worker_id: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: JobAttemptStatus,
    pub error: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobAttemptStatus {
    Started,
    Completed,
    Failed,
}
