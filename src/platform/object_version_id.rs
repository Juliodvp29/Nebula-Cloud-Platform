use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ObjectVersionId(pub Uuid);

impl ObjectVersionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for ObjectVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for ObjectVersionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ObjectVersionId> for Uuid {
    fn from(id: ObjectVersionId) -> Self {
        id.0
    }
}
