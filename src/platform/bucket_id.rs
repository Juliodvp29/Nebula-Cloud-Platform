use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BucketId(pub Uuid);

impl BucketId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for BucketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for BucketId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<BucketId> for Uuid {
    fn from(id: BucketId) -> Self {
        id.0
    }
}
