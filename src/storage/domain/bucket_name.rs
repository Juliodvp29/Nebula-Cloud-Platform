use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BucketName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BucketNameError {
    Empty,
    TooLong,
    InvalidCharacters,
}

impl BucketName {
    pub fn new(value: impl Into<String>) -> Result<Self, BucketNameError> {
        let value = value.into();

        if value.is_empty() {
            return Err(BucketNameError::Empty);
        }

        if value.len() > 63 {
            return Err(BucketNameError::TooLong);
        }

        if !value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        }) {
            return Err(BucketNameError::InvalidCharacters);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BucketName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_bucket_name() {
        let name = BucketName::new("my-bucket-123").unwrap();

        assert_eq!(name.as_str(), "my-bucket-123");
    }

    #[test]
    fn rejects_empty_bucket_name() {
        assert_eq!(BucketName::new(""), Err(BucketNameError::Empty));
    }

    #[test]
    fn rejects_uppercase_characters() {
        assert_eq!(
            BucketName::new("My-Bucket"),
            Err(BucketNameError::InvalidCharacters)
        );
    }

    #[test]
    fn rejects_spaces() {
        assert_eq!(
            BucketName::new("my bucket"),
            Err(BucketNameError::InvalidCharacters)
        );
    }

    #[test]
    fn rejects_names_longer_than_63_characters() {
        let name = "a".repeat(64);

        assert_eq!(BucketName::new(name), Err(BucketNameError::TooLong));
    }
}
