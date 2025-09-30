use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub owner_id: Uuid,
    pub created_at: i64, 
    pub updated_at: i64,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

impl Project {
    pub fn new(name: String, description: Option<String>, owner_id: Uuid, created_by: Uuid) -> Self {
        let now = current_timestamp();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            status: ProjectStatus::default(),
            owner_id,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Active,
    Archived,
    Completed,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Active
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "Active"),
            ProjectStatus::Archived => write!(f, "Archived"),
            ProjectStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Project '{}' [{}] - {}",
            self.name,
            self.status,
            self.description.as_deref().unwrap_or("No description")
        )
    }
}
