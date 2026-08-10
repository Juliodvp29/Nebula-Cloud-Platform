use crate::platform::{FunctionId, FunctionVersionId, ResourceId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct FunctionVersion {
    pub id: FunctionVersionId,
    pub function_id: FunctionId,
    pub resource_id: ResourceId,
    pub version: String,
    pub runtime: super::function::Runtime,
    pub handler: String,
    pub source_type: SourceType,
    pub source_location: Option<String>,
    pub image: Option<String>,
    pub code_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    Archive,
    ContainerImage,
}
