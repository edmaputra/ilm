use actix_web::{test, web, App};
use uuid::Uuid;
use serde_json::json;

use ilm::presentation::handlers::{ProjectHandler, get_project};
use ilm::application::ProjectService;
use ilm::infrastructure::DatabaseProjectRepository;
use ilm::domain::{Project, AppResult, AppError};
use ilm::application::repository::ProjectRepository;

// Mock repository for testing
#[derive(Clone)]
struct MockProjectRepository {
    projects: Vec<Project>,
}

impl MockProjectRepository {
    fn new() -> Self {
        let project = Project::new(
            "Test Project".to_string(),
            "Test Description".to_string(),
            "stage_1".to_string(),
            "user_1".to_string(),
        );
        
        Self {
            projects: vec![project],
        }
    }
}

#[async_trait::async_trait]
impl ProjectRepository for MockProjectRepository {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Project> {
        self.projects
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or(AppError::NotFound)
    }

    async fn create(&self, _project: &Project) -> AppResult<()> {
        Ok(())
    }

    async fn update(&self, _project: &Project) -> AppResult<()> {
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> AppResult<()> {
        Ok(())
    }
}

#[actix_web::test]
async fn test_get_project_success() {
    let mock_repo = std::sync::Arc::new(MockProjectRepository::new());
    let project_id = mock_repo.projects[0].id;
    
    let service = ProjectService::new(mock_repo);
    let handler = ProjectHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/projects", web::get().to(get_project))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/projects?id={}", project_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_project_not_found() {
    let mock_repo = std::sync::Arc::new(MockProjectRepository::new());
    let non_existent_id = Uuid::new_v4();
    
    let service = ProjectService::new(mock_repo);
    let handler = ProjectHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/projects", web::get().to(get_project))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/projects?id={}", non_existent_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_project_invalid_id() {
    let mock_repo = std::sync::Arc::new(MockProjectRepository::new());
    let service = ProjectService::new(mock_repo);
    let handler = ProjectHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/projects", web::get().to(get_project))
    ).await;

    let req = test::TestRequest::get()
        .uri("/projects?id=invalid-uuid")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
