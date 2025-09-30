use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Project, Task, AppResult};

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Project>;
    async fn create(&self, project: &Project) -> AppResult<()>;
    async fn update(&self, project: &Project) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Task>;
    async fn get_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>>;
    async fn create(&self, task: &Task) -> AppResult<()>;
    async fn update(&self, task: &Task) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
