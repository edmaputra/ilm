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
use infrastructure::repository::DatabaseProjectRepository;
use application::project::ProjectService;
use presentation::handlers::ProjectHandler;

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

    let server_address = format!("{}:{}", config.server.host, config.server.port);
    
    tracing::info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(project_handler.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/projects", web::get().to(presentation::handlers::get_project))
            )
    })
    .bind(&server_address)?
    .run()
    .await
}
