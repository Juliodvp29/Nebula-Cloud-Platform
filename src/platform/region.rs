use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Region(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionError;

impl Region {
    pub fn new(value: impl Into<String>) -> Result<Self, RegionError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RegionError);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Region {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_region() {
        assert!(Region::new("").is_err());
    }

    #[test]
    fn accepts_valid_region() {
        let region = Region::new("us-east-1").unwrap();

        assert_eq!(region.as_str(), "us-east-1");
    }
}
