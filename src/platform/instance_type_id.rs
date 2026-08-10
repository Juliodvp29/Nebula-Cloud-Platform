use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct InstanceTypeId(pub Uuid);

impl InstanceTypeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for InstanceTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for InstanceTypeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<InstanceTypeId> for Uuid {
    fn from(id: InstanceTypeId) -> Self {
        id.0
    }
}
