use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Task, AppResult, AppError};
use crate::application::repository::TaskRepository;

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
