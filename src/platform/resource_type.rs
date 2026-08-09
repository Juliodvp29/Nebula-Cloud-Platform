use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    S3Bucket,
    ComputeInstance,
    LambdaFunction,
}

impl ResourceType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::S3Bucket => "s3.bucket",
            Self::ComputeInstance => "compute.instance",
            Self::LambdaFunction => "lambda.function",
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_types_match_database_values() {
        assert_eq!(ResourceType::S3Bucket.as_str(), "s3.bucket");
        assert_eq!(ResourceType::ComputeInstance.as_str(), "compute.instance");
        assert_eq!(ResourceType::LambdaFunction.as_str(), "lambda.function");
    }

    #[test]
    fn resource_type_display_matches_database_value() {
        assert_eq!(ResourceType::S3Bucket.to_string(), "s3.bucket");
    }
}
