use crate::{
    error::AppError,
    platform::{CloudId, OrganizationId, Region, ResourceId, UserId},
    storage::domain::{
        bucket::Bucket, bucket_name::BucketName, bucket_repository::BucketRepository,
    },
};

pub struct CreateBucketCommand {
    pub cloud_id: CloudId,
    pub name: BucketName,
    pub organization_id: OrganizationId,
    pub created_by: UserId,
    pub region: Region,
}

pub struct CreateBucketUseCase<R> {
    repository: R,
}

impl<R> CreateBucketUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CreateBucketUseCase<R>
where
    R: BucketRepository,
{
    pub async fn execute(&self, command: CreateBucketCommand) -> Result<Bucket, AppError> {
        let existing_bucket = self.repository.find_by_name(command.name.as_str()).await?;

        if existing_bucket.is_some() {
            return Err(AppError::Conflict("bucket already exists".to_string()));
        }

        let bucket = Bucket::new(
            ResourceId::new(),
            command.cloud_id,
            command.name,
            command.organization_id,
            command.region,
            command.created_by,
        );

        self.repository.create(&bucket).await?;

        Ok(bucket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;

    use crate::{
        error::AppError,
        platform::{CloudId, OrganizationId, Region, ResourceId, UserId},
        storage::domain::{
            bucket::Bucket, bucket_name::BucketName, bucket_repository::BucketRepository,
            bucket_status::BucketStatus,
        },
    };

    struct InMemoryBucketRepository {
        buckets: std::sync::Mutex<Vec<Bucket>>,
    }

    impl InMemoryBucketRepository {
        fn new() -> Self {
            Self {
                buckets: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl BucketRepository for InMemoryBucketRepository {
        async fn create(&self, bucket: &Bucket) -> Result<(), AppError> {
            self.buckets
                .lock()
                .map_err(|_| AppError::Internal)?
                .push(bucket.clone());

            Ok(())
        }

        async fn find_by_id(&self, id: ResourceId) -> Result<Option<Bucket>, AppError> {
            let buckets = self.buckets.lock().map_err(|_| AppError::Internal)?;

            Ok(buckets.iter().find(|bucket| bucket.id() == id).cloned())
        }

        async fn find_by_name(&self, name: &str) -> Result<Option<Bucket>, AppError> {
            let buckets = self.buckets.lock().map_err(|_| AppError::Internal)?;

            Ok(buckets
                .iter()
                .find(|bucket| bucket.name().as_str() == name)
                .cloned())
        }
    }

    #[tokio::test]
    async fn creates_bucket() {
        let repository = InMemoryBucketRepository::new();
        let use_case = CreateBucketUseCase::new(repository);

        let organization_id = OrganizationId::new();
        let created_by = UserId::new();

        let command = CreateBucketCommand {
            cloud_id: CloudId::new("nbla-bucket-123").unwrap(),
            name: BucketName::new("my-bucket").unwrap(),
            organization_id,
            created_by,
            region: Region::new("us-east-1").unwrap(),
        };

        let bucket = use_case.execute(command).await.unwrap();

        assert_eq!(bucket.name().as_str(), "my-bucket");
        assert_eq!(bucket.organization_id(), organization_id);
        assert_eq!(bucket.created_by(), created_by);
        assert_eq!(bucket.region().as_str(), "us-east-1");
        assert_eq!(bucket.status(), BucketStatus::Active);
        assert!(!bucket.versioning_enabled());
    }

    #[tokio::test]
    async fn rejects_duplicate_bucket() {
        let repository = InMemoryBucketRepository::new();
        let use_case = CreateBucketUseCase::new(repository);

        let organization_id = OrganizationId::new();
        let created_by = UserId::new();

        let command = CreateBucketCommand {
            cloud_id: CloudId::new("nbla-bucket-123").unwrap(),
            name: BucketName::new("my-bucket").unwrap(),
            organization_id,
            created_by,
            region: Region::new("us-east-1").unwrap(),
        };

        use_case.execute(command).await.unwrap();

        let duplicate_command = CreateBucketCommand {
            cloud_id: CloudId::new("nbla-bucket-456").unwrap(),
            name: BucketName::new("my-bucket").unwrap(),
            organization_id,
            created_by,
            region: Region::new("us-east-1").unwrap(),
        };

        let result = use_case.execute(duplicate_command).await;

        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn different_orgs_same_name_is_rejected() {
        let repository = InMemoryBucketRepository::new();
        let use_case = CreateBucketUseCase::new(repository);

        let command = CreateBucketCommand {
            cloud_id: CloudId::new("nbla-bucket-123").unwrap(),
            name: BucketName::new("my-bucket").unwrap(),
            organization_id: OrganizationId::new(),
            created_by: UserId::new(),
            region: Region::new("us-east-1").unwrap(),
        };

        use_case.execute(command).await.unwrap();

        let duplicate_command = CreateBucketCommand {
            cloud_id: CloudId::new("nbla-bucket-456").unwrap(),
            name: BucketName::new("my-bucket").unwrap(),
            organization_id: OrganizationId::new(), // different org
            created_by: UserId::new(),
            region: Region::new("us-east-1").unwrap(),
        };

        // Should be rejected due to global uniqueness
        let result = use_case.execute(duplicate_command).await;

        assert!(matches!(result, Err(AppError::Conflict(_))));
    }
}
