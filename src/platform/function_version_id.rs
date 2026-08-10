use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FunctionVersionId(pub Uuid);

impl FunctionVersionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for FunctionVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for FunctionVersionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<FunctionVersionId> for Uuid {
    fn from(id: FunctionVersionId) -> Self {
        id.0
    }
}
