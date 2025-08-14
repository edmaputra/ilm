use std::sync::Arc;
use uuid::Uuid;
use crate::domain::{Project, AppResult};
use crate::application::repository::ProjectRepository;

#[derive(Clone)]
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_project(&self, id: Uuid) -> AppResult<Project> {
        self.repository.get_by_id(id).await
    }

    pub async fn create_project(
        &self,
        name: String,
        description: String,
        flow_stages_id: String,
        created_by: String,
    ) -> AppResult<Project> {
        let project = Project::new(name, description, flow_stages_id, created_by);
        self.repository.create(&project).await?;
        Ok(project)
    }

    pub async fn update_project(&self, project: &Project) -> AppResult<()> {
        // Check if project exists
        self.repository.get_by_id(project.id).await?;
        self.repository.update(project).await
    }

    pub async fn delete_project(&self, id: Uuid) -> AppResult<()> {
        // Check if project exists
        self.repository.get_by_id(id).await?;
        self.repository.delete(id).await
    }
}
