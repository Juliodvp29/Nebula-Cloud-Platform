use crate::error::AppError;
use crate::platform::{
    BucketId, MultipartUploadId, MultipartUploadPartId, ObjectId, ObjectVersionId, OrganizationId,
    Region, ResourceId,
};
use crate::s3::domain::bucket::Bucket;
use crate::s3::domain::multipart_upload::{
    MultipartUpload, MultipartUploadPart, MultipartUploadStatus,
};
use crate::s3::domain::object::Object;
use crate::s3::domain::object_version::ObjectVersion;
use async_trait::async_trait;

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn create(&self, bucket: &Bucket) -> Result<(), AppError>;
    async fn find_by_id(&self, id: BucketId) -> Result<Option<Bucket>, AppError>;
    async fn find_by_resource_id(
        &self,
        resource_id: ResourceId,
    ) -> Result<Option<Bucket>, AppError>;
    async fn find_by_name(
        &self,
        organization_id: OrganizationId,
        region: Region,
        name: &str,
    ) -> Result<Option<Bucket>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Bucket>, AppError>;
    async fn update(&self, bucket: &Bucket) -> Result<(), AppError>;
    async fn soft_delete(&self, id: BucketId) -> Result<(), AppError>;
}

#[async_trait]
pub trait ObjectRepository: Send + Sync {
    async fn create(&self, object: &Object) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Object>, AppError>;
    async fn find_by_bucket_and_key(
        &self,
        bucket_id: BucketId,
        key: &str,
    ) -> Result<Option<Object>, AppError>;
    async fn find_by_bucket(&self, bucket_id: BucketId) -> Result<Vec<Object>, AppError>;
    async fn find_by_bucket_prefix(
        &self,
        bucket_id: BucketId,
        prefix: &str,
    ) -> Result<Vec<Object>, AppError>;
    async fn update(&self, object: &Object) -> Result<(), AppError>;
    async fn soft_delete(&self, id: ObjectId) -> Result<(), AppError>;
}

#[async_trait]
pub trait ObjectVersionRepository: Send + Sync {
    async fn create(&self, version: &ObjectVersion) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ObjectVersionId) -> Result<Option<ObjectVersion>, AppError>;
    async fn find_by_object(&self, object_id: ObjectId) -> Result<Vec<ObjectVersion>, AppError>;
    async fn find_by_object_and_version(
        &self,
        object_id: ObjectId,
        version_id: &str,
    ) -> Result<Option<ObjectVersion>, AppError>;
    async fn find_latest(&self, object_id: ObjectId) -> Result<Option<ObjectVersion>, AppError>;
    async fn update_latest_flag(
        &self,
        object_id: ObjectId,
        version_id: ObjectVersionId,
    ) -> Result<(), AppError>;
    async fn soft_delete(&self, id: ObjectVersionId) -> Result<(), AppError>;
}

#[async_trait]
pub trait MultipartUploadRepository: Send + Sync {
    async fn create(&self, upload: &MultipartUpload) -> Result<(), AppError>;
    async fn find_by_id(&self, id: MultipartUploadId) -> Result<Option<MultipartUpload>, AppError>;
    async fn find_by_upload_id(&self, upload_id: &str)
    -> Result<Option<MultipartUpload>, AppError>;
    async fn find_by_bucket_and_key(
        &self,
        bucket_id: BucketId,
        object_key: &str,
    ) -> Result<Option<MultipartUpload>, AppError>;
    async fn update(&self, upload: &MultipartUpload) -> Result<(), AppError>;
    async fn update_status(
        &self,
        id: MultipartUploadId,
        status: MultipartUploadStatus,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait MultipartUploadPartRepository: Send + Sync {
    async fn create(&self, part: &MultipartUploadPart) -> Result<(), AppError>;
    async fn find_by_id(
        &self,
        id: MultipartUploadPartId,
    ) -> Result<Option<MultipartUploadPart>, AppError>;
    async fn find_by_upload(
        &self,
        upload_id: MultipartUploadId,
    ) -> Result<Vec<MultipartUploadPart>, AppError>;
    async fn find_by_upload_and_part(
        &self,
        upload_id: MultipartUploadId,
        part_number: u32,
    ) -> Result<Option<MultipartUploadPart>, AppError>;
}
