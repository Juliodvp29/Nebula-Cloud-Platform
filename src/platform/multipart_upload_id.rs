use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MultipartUploadId(pub Uuid);

impl MultipartUploadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for MultipartUploadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for MultipartUploadId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MultipartUploadId> for Uuid {
    fn from(id: MultipartUploadId) -> Self {
        id.0
    }
}
