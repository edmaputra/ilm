use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
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

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Task '{}' [{}] - Priority: {}",
            self.title,
            self.status,
            self.priority
        )
    }
}
// Helper function to get current Unix timestamp in milliseconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
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
    pub due_date: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        project_id: Uuid,
        assignee_id: Option<Uuid>,
        due_date: Option<i64>,
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
            updated_by: created_by,
        }
    }
}