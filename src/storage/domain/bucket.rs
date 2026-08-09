use chrono::{DateTime, Utc};

use crate::{
    platform::{CloudId, OrganizationId, Region, ResourceId, UserId},
    storage::domain::{bucket_name::BucketName, bucket_status::BucketStatus},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    id: ResourceId,
    cloud_id: CloudId,
    name: BucketName,
    organization_id: OrganizationId,
    region: Region,
    status: BucketStatus,
    versioning_enabled: bool,
    created_by: UserId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Bucket {
    pub fn new(
        id: ResourceId,
        cloud_id: CloudId,
        name: BucketName,
        organization_id: OrganizationId,
        region: Region,
        created_by: UserId,
    ) -> Self {
        let now = Utc::now();

        Self {
            id,
            cloud_id,
            name,
            organization_id,
            region,
            status: BucketStatus::Active,
            versioning_enabled: false,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }

    pub fn cloud_id(&self) -> &CloudId {
        &self.cloud_id
    }

    pub fn name(&self) -> &BucketName {
        &self.name
    }

    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    pub fn region(&self) -> &Region {
        &self.region
    }

    pub fn status(&self) -> BucketStatus {
        self.status
    }

    pub fn versioning_enabled(&self) -> bool {
        self.versioning_enabled
    }

    pub fn created_by(&self) -> UserId {
        self.created_by
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn enable_versioning(&mut self) {
        self.versioning_enabled = true;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{CloudId, OrganizationId, Region, ResourceId, UserId};

    #[test]
    fn new_bucket_starts_with_versioning_disabled() {
        let bucket = Bucket::new(
            ResourceId::new(),
            CloudId::new("nbla-bucket-123").unwrap(),
            BucketName::new("my-bucket").unwrap(),
            OrganizationId::new(),
            Region::new("us-east-1").unwrap(),
            UserId::new(),
        );

        assert!(!bucket.versioning_enabled());
    }

    #[test]
    fn enabling_versioning_updates_bucket_state() {
        let mut bucket = Bucket::new(
            ResourceId::new(),
            CloudId::new("nbla-bucket-123").unwrap(),
            BucketName::new("my-bucket").unwrap(),
            OrganizationId::new(),
            Region::new("us-east-1").unwrap(),
            UserId::new(),
        );

        bucket.enable_versioning();

        assert!(bucket.versioning_enabled());
    }

    #[test]
    fn bucket_status_is_active_by_default() {
        let bucket = Bucket::new(
            ResourceId::new(),
            CloudId::new("nbla-bucket-123").unwrap(),
            BucketName::new("my-bucket").unwrap(),
            OrganizationId::new(),
            Region::new("us-east-1").unwrap(),
            UserId::new(),
        );

        assert_eq!(bucket.status(), BucketStatus::Active);
    }
}
