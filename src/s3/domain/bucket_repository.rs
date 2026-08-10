use async_trait::async_trait;

use crate::{error::AppError, platform::ResourceId};

use super::bucket::Bucket;

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn create(&self, bucket: &Bucket) -> Result<(), AppError>;

    async fn find_by_id(&self, id: ResourceId) -> Result<Option<Bucket>, AppError>;

    async fn find_by_name(&self, name: &str) -> Result<Option<Bucket>, AppError>;
}
