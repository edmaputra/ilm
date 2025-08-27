use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Project, Task, AppResult, AppError};
use crate::application::repository::{ProjectRepository, TaskRepository};

#[derive(Clone)]
pub struct DatabaseProjectRepository {
    pool: PgPool,
}

impl DatabaseProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for DatabaseProjectRepository {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Project> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, name, description, flow_stages_id, created_at, created_by, updated_at 
             FROM project WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        project.ok_or(AppError::NotFound)
    }

    async fn create(&self, project: &Project) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO project (id, name, description, flow_stages_id, created_at, created_by, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.flow_stages_id)
        .bind(project.created_at)
        .bind(&project.created_by)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, project: &Project) -> AppResult<()> {
        let rows_affected = sqlx::query(
            "UPDATE project 
             SET name = $2, description = $3, flow_stages_id = $4, updated_at = $5
             WHERE id = $1"
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.flow_stages_id)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let rows_affected = sqlx::query("DELETE FROM project WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct DatabaseTaskRepository {
    pool: PgPool,
}

impl DatabaseTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for DatabaseTaskRepository {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Task> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, project_id, title, description, status, priority, assigned_to, due_date, created_at, created_by, updated_at 
             FROM task WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        task.ok_or(AppError::NotFound)
    }

    async fn get_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, project_id, title, description, status, priority, assigned_to, due_date, created_at, created_by, updated_at 
             FROM task WHERE project_id = $1 ORDER BY created_at DESC"
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn create(&self, task: &Task) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO task (id, project_id, title, description, status, priority, assigned_to, due_date, created_at, created_by, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
        )
        .bind(task.id)
        .bind(task.project_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.assigned_to)
        .bind(task.due_date)
        .bind(task.created_at)
        .bind(&task.created_by)
        .bind(task.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, task: &Task) -> AppResult<()> {
        let rows_affected = sqlx::query(
            "UPDATE task 
             SET title = $2, description = $3, status = $4, priority = $5, assigned_to = $6, due_date = $7, updated_at = $8
             WHERE id = $1"
        )
        .bind(task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.assigned_to)
        .bind(task.due_date)
        .bind(task.updated_at)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let rows_affected = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
