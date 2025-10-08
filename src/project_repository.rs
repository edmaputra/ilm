use sqlx::{SqlitePool, Error as SqlxError, FromRow};
use uuid::Uuid;
use crate::project::{Project, ProjectStatus};

// Database record struct for cleaner mapping
#[derive(FromRow)]
struct ProjectRecord {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub owner_id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

// Helper function to parse project status string (standalone function)
fn parse_project_status(status_str: &str) -> ProjectStatus {
    match status_str {
        "Active" => ProjectStatus::Active,
        "Archived" => ProjectStatus::Archived,
        "Completed" => ProjectStatus::Completed,
        _ => ProjectStatus::Active, // Default fallback
    }
}

// Implement From trait for automatic conversion from ProjectRecord to Project
impl From<&ProjectRecord> for Project {
    fn from(record: &ProjectRecord) -> Self {
        Project {
            id: record.id,
            name: record.name.clone(),
            description: record.description.clone(),
            status: parse_project_status(&record.status),
            owner_id: record.owner_id,
            created_at: record.created_at,
            updated_at: record.updated_at,
            created_by: record.created_by,
            updated_by: record.updated_by,
        }
    }
}

// Also implement From for owned ProjectRecord
impl From<ProjectRecord> for Project {
    fn from(record: ProjectRecord) -> Self {
        Project {
            id: record.id,
            name: record.name,
            description: record.description,
            status: parse_project_status(&record.status),
            owner_id: record.owner_id,
            created_at: record.created_at,
            updated_at: record.updated_at,
            created_by: record.created_by,
            updated_by: record.updated_by,
        }
    }
}

pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_project(&self, project: &Project) -> Result<(), SqlxError> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, description, status, owner_id, created_at, updated_at, created_by, updated_by)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(project.status.to_string())
        .bind(project.owner_id)
        .bind(project.created_at)
        .bind(project.updated_at)
        .bind(project.created_by)
        .bind(project.updated_by)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_project_by_name(&self, project_name: &str) -> Result<Option<Project>, SqlxError> {
        let record = sqlx::query_as::<_, ProjectRecord>(
            r#"
            SELECT id, name, description, status, owner_id, created_at, updated_at, created_by, updated_by
            FROM projects
            WHERE name = ?
            "#
        )
        .bind(project_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|rec| rec.into()))
    }

    // pub async fn update_project(&self, project: &Project) -> Result<(), SqlxError> {
    //     sqlx::query!(
    //         r#"
    //         UPDATE projects 
    //         SET name = ?, description = ?, status = ?, owner_id = ?, updated_at = ?, updated_by = ?
    //         WHERE id = ?
    //         "#,
    //         project.name,
    //         project.description,
    //         project.status.to_string(),
    //         project.owner_id,
    //         project.updated_at,
    //         project.updated_by,
    //         project.id
    //     )
    //     .execute(&self.pool)
    //     .await?;
    //     Ok(())
    // }

    // pub async fn delete_project(&self, project_id: Uuid) -> Result<bool, SqlxError> {
    //     let result = sqlx::query!(
    //         r#"
    //         DELETE FROM projects WHERE id = ?
    //         "#,
    //         project_id
    //     )
    //     .execute(&self.pool)
    //     .await?;

    //     Ok(result.rows_affected() > 0)
    // }

    // pub async fn get_projects_by_owner(&self, owner_id: Uuid) -> Result<Vec<Project>, SqlxError> {
    //     let records = sqlx::query_as::<_, ProjectRecord>(
    //         r#"
    //         SELECT id, name, description, status, owner_id, created_at, updated_at, created_by, updated_by
    //         FROM projects
    //         WHERE owner_id = ?
    //         ORDER BY created_at DESC
    //         "#,
    //         owner_id
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;

    //     let projects = records
    //         .into_iter()
    //         .map(|rec| rec.into())
    //         .collect();

    //     Ok(projects)
    // }
}