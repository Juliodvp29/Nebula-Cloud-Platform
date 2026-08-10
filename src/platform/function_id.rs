use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FunctionId(pub Uuid);

impl FunctionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for FunctionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<FunctionId> for Uuid {
    fn from(id: FunctionId) -> Self {
        id.0
    }
}
