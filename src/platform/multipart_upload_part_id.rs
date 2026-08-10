use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MultipartUploadPartId(pub Uuid);

impl MultipartUploadPartId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for MultipartUploadPartId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for MultipartUploadPartId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MultipartUploadPartId> for Uuid {
    fn from(id: MultipartUploadPartId) -> Self {
        id.0
    }
}
