use crate::platform::{ObjectId, ObjectVersionId, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ObjectVersion {
    pub id: ObjectVersionId,
    pub object_id: ObjectId,
    pub resource_id: ResourceId,
    pub version_id: String,
    pub storage_key: String,
    pub size_bytes: u64,
    pub etag: String,
    pub metadata: serde_json::Value,
    pub is_latest: bool,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
