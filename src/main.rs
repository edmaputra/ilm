use uuid::Uuid;

mod db;
mod project;
mod project_repository;
mod project_service;
mod task;

use db::setup_database;
use project::Project;
use project_repository::ProjectRepository;
use project_service::ProjectService;

#[tokio::main ]
async fn main() {
    run().await;
}

pub async fn run() {
    let pool = setup_database("sqlite://ilm.db").await.expect("Failed to setup database");

    let project_repository = ProjectRepository::new(pool.clone());
    let project_service = ProjectService::new(project_repository);

    let owner_id = Uuid::new_v4();
    let project = Project::new(
        "Web Application Redesign".to_string(),
        Some("Complete redesign of the company website with modern UI/UX".to_string()),
        owner_id,
        owner_id, // created_by is the owner
    );

    match project_service.create_project(&project).await {
        Ok(_) => println!("Project created successfully"),
        Err(e) => eprintln!("Failed to create project: {}", e),
    }

    match project_service.get_project_by_name(&project.name).await {
        Ok(project) => println!("Project found: {:?}", project),
        Err(e) => eprintln!("Failed to get project. Message: {}", e),
    }

}
