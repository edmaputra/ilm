use actix_web::{test, web, App};
use uuid::Uuid;
use serde_json::json;

use ilm::presentation::handlers::{ProjectHandler, TaskHandler, get_project, get_task, get_tasks_by_project};
use ilm::application::{ProjectService, TaskService};
use ilm::infrastructure::{DatabaseProjectRepository, DatabaseTaskRepository};
use ilm::domain::{Project, Task, TaskStatus, TaskPriority, AppResult, AppError};
use ilm::application::repository::{ProjectRepository, TaskRepository};

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

// Mock task repository for testing
#[derive(Clone)]
struct MockTaskRepository {
    tasks: Vec<Task>,
}

impl MockTaskRepository {
    fn new() -> Self {
        let project_id = Uuid::new_v4();
        let task = Task::new(
            project_id,
            "Test Task".to_string(),
            Some("Test task description".to_string()),
            TaskPriority::High,
            Some("user_1".to_string()),
            None,
            "creator_1".to_string(),
        );
        
        Self {
            tasks: vec![task],
        }
    }
}

#[async_trait::async_trait]
impl TaskRepository for MockTaskRepository {
    async fn get_by_id(&self, id: Uuid) -> AppResult<Task> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or(AppError::NotFound)
    }

    async fn get_by_project_id(&self, project_id: Uuid) -> AppResult<Vec<Task>> {
        let tasks = self.tasks
            .iter()
            .filter(|t| t.project_id == project_id)
            .cloned()
            .collect();
        Ok(tasks)
    }

    async fn create(&self, _task: &Task) -> AppResult<()> {
        Ok(())
    }

    async fn update(&self, _task: &Task) -> AppResult<()> {
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> AppResult<()> {
        Ok(())
    }
}

#[actix_web::test]
async fn test_get_task_success() {
    let mock_repo = std::sync::Arc::new(MockTaskRepository::new());
    let task_id = mock_repo.tasks[0].id;
    
    let service = TaskService::new(mock_repo);
    let handler = TaskHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/tasks", web::get().to(get_task))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/tasks?id={}", task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_tasks_by_project_success() {
    let mock_repo = std::sync::Arc::new(MockTaskRepository::new());
    let project_id = mock_repo.tasks[0].project_id;
    
    let service = TaskService::new(mock_repo);
    let handler = TaskHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/projects/tasks", web::get().to(get_tasks_by_project))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/projects/tasks?project_id={}", project_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_task_not_found() {
    let mock_repo = std::sync::Arc::new(MockTaskRepository::new());
    let non_existent_id = Uuid::new_v4();
    
    let service = TaskService::new(mock_repo);
    let handler = TaskHandler::new(service);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(handler))
            .route("/tasks", web::get().to(get_task))
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/tasks?id={}", non_existent_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
