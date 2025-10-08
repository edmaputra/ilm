use crate::project_repository::ProjectRepository;
use crate::project::Project;

pub struct ProjectService {
    repository: ProjectRepository,
}

impl ProjectService {

    pub fn new(repository: ProjectRepository) -> Self {
        Self { repository }
    }

    // Example method to get a project by name
    pub async fn get_project_by_name(&self, project_name: &str) -> Result<Project, sqlx::Error> {
        self.repository.get_project_by_name(project_name).await
            .and_then(|opt| opt.ok_or(sqlx::Error::RowNotFound))
    }

    pub async fn create_project(&self, project: &Project) -> Result<bool, sqlx::Error> {
        self.repository.create_project(project).await?;
        Ok(true)
    }



}