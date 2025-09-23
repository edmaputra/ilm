use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use time::{OffsetDateTime, format_description};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Todo
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "To Do"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Blocked => write!(f, "Blocked"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Urgent => write!(f, "Urgent"),
        }
    }
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

fn timestamp_to_string_compact(timestamp_ms: i64) -> String {
    let datetime = OffsetDateTime::from_unix_timestamp_nanos((timestamp_ms * 1_000_000) as i128)
        .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
    
    let format = format_description::parse("[day][month][year]").unwrap();
    
    datetime.format(&format).unwrap_or_else(|_| "Invalid".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub project_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<i64>,       // Optional deadline (epoch milliseconds)
    pub created_at: i64,             // Creation timestamp (epoch milliseconds)
    pub updated_at: i64,             // Last update timestamp (epoch milliseconds)
    pub created_by: Uuid,            // Who created this task
    pub updated_by: Uuid,            // Who last updated this task
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        project_id: Uuid,
        assignee_id: Option<Uuid>,
        due_date: Option<i64>, // Now accepts Unix timestamp in milliseconds
        created_by: Uuid,
    ) -> Self {
        let now = current_timestamp();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::default(),
            priority: TaskPriority::default(),
            project_id,
            assignee_id,
            due_date,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by, // Initially, creator is also the updater
        }
    }

    pub fn update_title(&mut self, title: String, updated_by: Uuid) {
        self.title = title;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn update_description(&mut self, description: Option<String>, updated_by: Uuid) {
        self.description = description;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn update_status(&mut self, status: TaskStatus, updated_by: Uuid) {
        self.status = status;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn update_priority(&mut self, priority: TaskPriority, updated_by: Uuid) {
        self.priority = priority;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn assign_to(&mut self, assignee_id: Option<Uuid>, updated_by: Uuid) {
        self.assignee_id = assignee_id;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn set_due_date(&mut self, due_date: Option<i64>, updated_by: Uuid) {
        self.due_date = due_date;
        self.updated_at = current_timestamp();
        self.updated_by = updated_by;
    }

    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Done
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            current_timestamp() > due_date && !self.is_completed()
        } else {
            false
        }
    }

    pub fn is_assigned(&self) -> bool {
        self.assignee_id.is_some()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Task title cannot be empty".to_string());
        }
        
        if self.title.len() > 200 {
            return Err("Task title cannot exceed 200 characters".to_string());
        }

        if let Some(desc) = &self.description {
            if desc.len() > 1000 {
                return Err("Task description cannot exceed 1000 characters".to_string());
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

    pub fn due_date_formatted(&self) -> Option<String> {
        self.due_date.map(timestamp_to_string)
    }

    // Get due date in compact ddMMyyyy format
    pub fn due_date_compact(&self) -> Option<String> {
        self.due_date.map(timestamp_to_string_compact)
    }

    pub fn age_in_milliseconds(&self) -> i64 {
        current_timestamp() - self.created_at
    }

    pub fn age_in_seconds(&self) -> i64 {
        self.age_in_milliseconds() / 1000
    }

    pub fn time_until_due_milliseconds(&self) -> Option<i64> {
        self.due_date.map(|due| due - current_timestamp())
    }

    pub fn time_until_due(&self) -> Option<i64> {
        self.time_until_due_milliseconds().map(|ms| ms / 1000)
    }

    pub fn is_due_soon(&self, hours: i64) -> bool {
        if let Some(time_left_ms) = self.time_until_due_milliseconds() {
            let time_left_seconds = time_left_ms / 1000;
            time_left_seconds > 0 && time_left_seconds <= (hours * 3600) // Convert hours to seconds
        } else {
            false
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Task '{}' [{}] - Priority: {} - {}",
            self.title,
            self.status,
            self.priority,
            if self.is_overdue() { "OVERDUE" } else { "On track" }
        )
    }
}