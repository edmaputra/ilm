use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub assigned_to: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl Task {
    pub fn new(
        project_id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        assigned_to: Option<String>,
        due_date: Option<DateTime<Utc>>,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            assigned_to,
            due_date,
            created_at: now,
            created_by,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn assign_to(&mut self, assigned_to: Option<String>) {
        self.assigned_to = assigned_to;
        self.updated_at = Utc::now();
    }

    pub fn set_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let project_id = Uuid::new_v4();
        let title = "Test Task".to_string();
        let description = Some("Test Description".to_string());
        let priority = TaskPriority::High;
        let assigned_to = Some("user_1".to_string());
        let due_date = Some(Utc::now());
        let created_by = "creator_1".to_string();

        let task = Task::new(
            project_id,
            title.clone(),
            description.clone(),
            priority.clone(),
            assigned_to.clone(),
            due_date,
            created_by.clone(),
        );

        assert_eq!(task.project_id, project_id);
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert!(matches!(task.status, TaskStatus::Todo));
        assert!(matches!(task.priority, TaskPriority::High));
        assert_eq!(task.assigned_to, assigned_to);
        assert_eq!(task.created_by, created_by);
        assert!(task.created_at <= Utc::now());
        assert!(task.updated_at <= Utc::now());
        assert_eq!(task.created_at, task.updated_at);
    }

    #[test]
    fn test_task_status_update() {
        let mut task = Task::new(
            Uuid::new_v4(),
            "Test Task".to_string(),
            None,
            TaskPriority::Medium,
            None,
            None,
            "creator_1".to_string(),
        );

        let original_updated_at = task.updated_at;
        
        // Small delay to ensure updated_at changes
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        task.update_status(TaskStatus::InProgress);
        
        assert!(matches!(task.status, TaskStatus::InProgress));
        assert!(task.updated_at > original_updated_at);
    }

    #[test]
    fn test_task_assignment() {
        let mut task = Task::new(
            Uuid::new_v4(),
            "Test Task".to_string(),
            None,
            TaskPriority::Low,
            None,
            None,
            "creator_1".to_string(),
        );

        task.assign_to(Some("assignee_1".to_string()));
        
        assert_eq!(task.assigned_to, Some("assignee_1".to_string()));
    }

    #[test]
    fn test_task_unique_ids() {
        let project_id = Uuid::new_v4();
        
        let task1 = Task::new(
            project_id,
            "Task 1".to_string(),
            None,
            TaskPriority::Medium,
            None,
            None,
            "creator_1".to_string(),
        );

        let task2 = Task::new(
            project_id,
            "Task 2".to_string(),
            None,
            TaskPriority::High,
            None,
            None,
            "creator_1".to_string(),
        );

        assert_ne!(task1.id, task2.id);
        assert_eq!(task1.project_id, task2.project_id);
    }
}
