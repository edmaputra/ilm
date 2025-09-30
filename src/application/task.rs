use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{Task, TaskStatus, TaskPriority, AppResult};
use crate::application::repository::TaskRepository;

#[derive(Clone)]
pub struct TaskService {
    repository: Arc<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_task(&self, id: Uuid) -> AppResult<Task> {
        self.repository.get_by_id(id).await
    }

    pub async fn get_tasks_by_project(&self, project_id: Uuid) -> AppResult<Vec<Task>> {
        self.repository.get_by_project_id(project_id).await
    }

    pub async fn create_task(
        &self,
        project_id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        assigned_to: Option<String>,
        due_date: Option<DateTime<Utc>>,
        created_by: String,
    ) -> AppResult<Task> {
        let task = Task::new(
            project_id,
            title,
            description,
            priority,
            assigned_to,
            due_date,
            created_by,
        );
        self.repository.create(&task).await?;
        Ok(task)
    }

    pub async fn update_task(&self, task: &Task) -> AppResult<()> {
        // Check if task exists
        self.repository.get_by_id(task.id).await?;
        self.repository.update(task).await
    }

    pub async fn update_task_status(&self, id: Uuid, status: TaskStatus) -> AppResult<Task> {
        let mut task = self.repository.get_by_id(id).await?;
        task.update_status(status);
        self.repository.update(&task).await?;
        Ok(task)
    }

    pub async fn assign_task(&self, id: Uuid, assigned_to: Option<String>) -> AppResult<Task> {
        let mut task = self.repository.get_by_id(id).await?;
        task.assign_to(assigned_to);
        self.repository.update(&task).await?;
        Ok(task)
    }

    pub async fn set_task_due_date(&self, id: Uuid, due_date: Option<DateTime<Utc>>) -> AppResult<Task> {
        let mut task = self.repository.get_by_id(id).await?;
        task.set_due_date(due_date);
        self.repository.update(&task).await?;
        Ok(task)
    }

    pub async fn delete_task(&self, id: Uuid) -> AppResult<()> {
        // Check if task exists
        self.repository.get_by_id(id).await?;
        self.repository.delete(id).await
    }
}
