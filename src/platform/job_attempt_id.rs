use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct JobAttemptId(pub Uuid);

impl JobAttemptId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for JobAttemptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for JobAttemptId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<JobAttemptId> for Uuid {
    fn from(id: JobAttemptId) -> Self {
        id.0
    }
}
