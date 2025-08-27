use actix_web::{web, App, HttpServer};
use std::{io, sync::Arc};
use tracing_actix_web::TracingLogger;

mod config;
mod domain;
mod infrastructure;
mod application;
mod presentation;

use config::AppConfig;
use infrastructure::database::Database;
use infrastructure::{DatabaseProjectRepository, DatabaseTaskRepository};
use application::{ProjectService, TaskService};
use presentation::project_handlers::{ProjectHandler, get_project};
use presentation::task_handlers::{TaskHandler, get_task, get_tasks_by_project, create_task, update_task_status, delete_task};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");
    
    // Initialize database
    let database = Database::new(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    // Setup dependencies
    let project_repository = Arc::new(DatabaseProjectRepository::new(database.pool().clone()));
    let project_service = ProjectService::new(project_repository);
    let project_handler = ProjectHandler::new(project_service);

    let task_repository = Arc::new(DatabaseTaskRepository::new(database.pool().clone()));
    let task_service = TaskService::new(task_repository);
    let task_handler = TaskHandler::new(task_service);

    let server_address = format!("{}:{}", config.server.host, config.server.port);
    
    tracing::info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(project_handler.clone()))
            .app_data(web::Data::new(task_handler.clone()))
            .service(
                web::scope("/api/v1")
                    // Project routes
                    .route("/projects", web::get().to(get_project))
                    // Task routes
                    .route("/tasks", web::get().to(get_task))
                    .route("/tasks", web::post().to(create_task))
                    .route("/tasks/status", web::put().to(update_task_status))
                    .route("/tasks", web::delete().to(delete_task))
                    .route("/projects/tasks", web::get().to(get_tasks_by_project))
            )
    })
    .bind(&server_address)?
    .run()
    .await
}
