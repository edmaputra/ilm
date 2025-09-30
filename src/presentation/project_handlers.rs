use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use serde_json::json;
use crate::application::ProjectService;
use crate::domain::AppError;

#[derive(Clone)]
pub struct ProjectHandler {
    service: ProjectService,
}

impl ProjectHandler {
    pub fn new(service: ProjectService) -> Self {
        Self { service }
    }
}

pub async fn get_project(
    handler: web::Data<ProjectHandler>,
    query: web::Query<ProjectQuery>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&query.id) {
        Ok(id) => {
            match handler.service.get_project(id).await {
                Ok(project) => Ok(HttpResponse::Ok().json(project)),
                Err(AppError::NotFound) => Ok(HttpResponse::NotFound().json(json!({
                    "error": "Project not found"
                }))),
                Err(err) => {
                    tracing::error!("Error getting project: {:?}", err);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Internal server error"
                    })))
                }
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid project ID format"
        })))
    }
}

#[derive(serde::Deserialize)]
pub struct ProjectQuery {
    pub id: String,
}
