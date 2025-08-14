use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Project, AppResult};

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Project>;
    async fn create(&self, project: &Project) -> AppResult<()>;
    async fn update(&self, project: &Project) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
