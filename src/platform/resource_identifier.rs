use std::fmt;

use super::cloud_id::CloudId;
use super::resource_type::ResourceType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceIdentifier(String);

impl ResourceIdentifier {
    pub fn new(resource_type: ResourceType, cloud_id: &CloudId) -> Self {
        Self(format!(
            "{}/{}",
            resource_type_prefix(resource_type),
            cloud_id.as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn resource_type_prefix(resource_type: ResourceType) -> &'static str {
    match resource_type {
        ResourceType::S3Bucket => "s3",
        ResourceType::ComputeInstance => "compute",
        ResourceType::LambdaFunction => "lambda",
    }
}

impl fmt::Display for ResourceIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_s3_identifier() {
        let cloud_id = CloudId::new("bucket-prod-001").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::S3Bucket, &cloud_id);

        assert_eq!(identifier.as_str(), "s3/bucket-prod-001");
    }

    #[test]
    fn generates_compute_identifier() {
        let cloud_id = CloudId::new("web-prod-001").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::ComputeInstance, &cloud_id);

        assert_eq!(identifier.as_str(), "compute/web-prod-001");
    }

    #[test]
    fn generates_lambda_identifier() {
        let cloud_id = CloudId::new("image-processor").unwrap();

        let identifier = ResourceIdentifier::new(ResourceType::LambdaFunction, &cloud_id);

        assert_eq!(identifier.as_str(), "lambda/image-processor");
    }
}
