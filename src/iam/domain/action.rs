use crate::platform::ActionId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Action {
    pub id: ActionId,
    pub service: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Action {
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.service, self.name)
    }
}
