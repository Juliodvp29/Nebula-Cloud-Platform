use crate::platform::{BucketId, MultipartUploadId, MultipartUploadPartId, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MultipartUpload {
    pub id: MultipartUploadId,
    pub bucket_id: BucketId,
    pub object_key: String,
    pub upload_id: String,
    pub status: MultipartUploadStatus,
    pub metadata: serde_json::Value,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub aborted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipartUploadStatus {
    Initiated,
    Completed,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct MultipartUploadPart {
    pub id: MultipartUploadPartId,
    pub multipart_upload_id: MultipartUploadId,
    pub part_number: u32,
    pub storage_key: String,
    pub size_bytes: u64,
    pub etag: String,
    pub created_at: DateTime<Utc>,
}
