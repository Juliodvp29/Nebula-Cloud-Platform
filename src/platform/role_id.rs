use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoleId(pub Uuid);

impl RoleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for RoleId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<RoleId> for Uuid {
    fn from(id: RoleId) -> Self {
        id.0
    }
}
