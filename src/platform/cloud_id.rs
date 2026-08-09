use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloudId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudIdError;

impl CloudId {
    pub fn new(value: impl Into<String>) -> Result<Self, CloudIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CloudIdError);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CloudId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_cloud_id() {
        assert!(CloudId::new("").is_err());
    }

    #[test]
    fn accepts_valid_cloud_id() {
        let id = CloudId::new("nbla-bucket-123").unwrap();

        assert_eq!(id.as_str(), "nbla-bucket-123");
    }
}
