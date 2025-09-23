use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use time::{OffsetDateTime, format_description};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub owner_id: Uuid,
    pub created_at: i64,         // Unix timestamp in milliseconds
    pub updated_at: i64,         // Unix timestamp in milliseconds
    pub created_by: Uuid,        // Who created this project
    pub updated_by: Uuid,        // Who last updated this project
}

// Helper function to get current Unix timestamp in milliseconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

// Helper function to format timestamp as human-readable string
fn timestamp_to_string(timestamp_ms: i64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos((timestamp_ms * 1_000_000) as i128)
        .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
    
    let format = format_description::parse(
        "[day]/[month]/[year] [hour]:[minute]:[second].[subsecond digits:3] UTC"
    ).unwrap();
    
    datetime.format(&format).unwrap_or_else(|_| "Invalid date".to_string())
}

// Helper function to convert days since epoch to ddMMyyyy format
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
            updated_by: created_by, // Initially, creator is also the updater
        }
    }

    pub fn update_name(&mut self, name: String, updated_by: Uuid) {
        self.name = name;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn update_description(&mut self, description: Option<String>, updated_by: Uuid) {
        self.description = description;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn update_status(&mut self, status: ProjectStatus, updated_by: Uuid) {
        self.status = status;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn is_active(&self) -> bool {
        self.status == ProjectStatus::Active
    }

    pub fn is_completed(&self) -> bool {
        self.status == ProjectStatus::Completed
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        
        if self.name.len() > 100 {
            return Err("Project name cannot exceed 100 characters".to_string());
        }

        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err("Project description cannot exceed 500 characters".to_string());
            }
        }

        Ok(())
    }

    // Utility methods for timestamp handling
    pub fn created_at_formatted(&self) -> String {
        timestamp_to_string(self.created_at)
    }

    pub fn updated_at_formatted(&self) -> String {  
        timestamp_to_string(self.updated_at)
    }

    pub fn age_in_milliseconds(&self) -> i64 {
        current_timestamp() - self.created_at
    }

    pub fn age_in_seconds(&self) -> i64 {
        self.age_in_milliseconds() / 1000
    }

    pub fn age_in_days(&self) -> i64 {
        self.age_in_seconds() / 86400 // 24 * 60 * 60
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