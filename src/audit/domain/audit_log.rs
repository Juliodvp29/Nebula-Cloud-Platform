use crate::platform::{AuditLogId, OrganizationId, ResourceId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: AuditLogId,
    pub organization_id: OrganizationId,
    pub actor_user_id: Option<UserId>,
    pub action: String,
    pub resource_id: Option<ResourceId>,
    pub resource_type: Option<String>,
    pub request_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub result: AuditResult,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditResult {
    Success,
    Denied,
    Failed,
}
