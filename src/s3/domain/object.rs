use crate::platform::{BucketId, ObjectId, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub bucket_id: BucketId,
    pub resource_id: ResourceId,
    pub key: String,
    pub size_bytes: u64,
    pub etag: String,
    pub content_type: Option<String>,
    pub storage_key: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
