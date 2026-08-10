use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct InvocationId(pub Uuid);

impl InvocationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for InvocationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for InvocationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<InvocationId> for Uuid {
    fn from(id: InvocationId) -> Self {
        id.0
    }
}
