use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub flow_stages_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        flow_stages_id: String,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            flow_stages_id,
            created_at: now,
            created_by,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let name = "Test Project".to_string();
        let description = "Test Description".to_string();
        let flow_stages_id = "stage_1".to_string();
        let created_by = "user_1".to_string();

        let project = Project::new(
            name.clone(),
            description.clone(),
            flow_stages_id.clone(),
            created_by.clone(),
        );

        assert_eq!(project.name, name);
        assert_eq!(project.description, description);
        assert_eq!(project.flow_stages_id, flow_stages_id);
        assert_eq!(project.created_by, created_by);
        assert!(project.created_at <= Utc::now());
        assert!(project.updated_at <= Utc::now());
        assert_eq!(project.created_at, project.updated_at);
    }

    #[test]
    fn test_project_unique_ids() {
        let project1 = Project::new(
            "Project 1".to_string(),
            "Description 1".to_string(),
            "stage_1".to_string(),
            "user_1".to_string(),
        );

        let project2 = Project::new(
            "Project 2".to_string(),
            "Description 2".to_string(),
            "stage_2".to_string(),
            "user_2".to_string(),
        );

        assert_ne!(project1.id, project2.id);
    }
}
