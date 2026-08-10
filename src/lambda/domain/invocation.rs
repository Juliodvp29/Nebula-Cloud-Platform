use crate::platform::{FunctionId, FunctionVersionId, InvocationId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Invocation {
    pub id: InvocationId,
    pub function_id: FunctionId,
    pub function_version_id: FunctionVersionId,
    pub request_id: String,
    pub status: InvocationStatus,
    pub trigger_type: TriggerType,
    pub payload_size: Option<u64>,
    pub result_size: Option<u64>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvocationStatus {
    Queued,
    Running,
    Success,
    Failed,
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    Http,
    Manual,
    Event,
    Schedule,
}
