use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use serde_json::json;
use chrono::{DateTime, Utc};
use crate::application::{ProjectService, TaskService};
use crate::domain::{AppError, TaskStatus, TaskPriority};

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

// Task handlers
#[derive(Clone)]
pub struct TaskHandler {
    service: TaskService,
}

impl TaskHandler {
    pub fn new(service: TaskService) -> Self {
        Self { service }
    }
}

pub async fn get_task(
    handler: web::Data<TaskHandler>,
    query: web::Query<TaskQuery>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&query.id) {
        Ok(id) => {
            match handler.service.get_task(id).await {
                Ok(task) => Ok(HttpResponse::Ok().json(task)),
                Err(AppError::NotFound) => Ok(HttpResponse::NotFound().json(json!({
                    "error": "Task not found"
                }))),
                Err(err) => {
                    tracing::error!("Error getting task: {:?}", err);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Internal server error"
                    })))
                }
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid task ID format"
        })))
    }
}

pub async fn get_tasks_by_project(
    handler: web::Data<TaskHandler>,
    query: web::Query<ProjectTasksQuery>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&query.project_id) {
        Ok(project_id) => {
            match handler.service.get_tasks_by_project(project_id).await {
                Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
                Err(err) => {
                    tracing::error!("Error getting tasks by project: {:?}", err);
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

pub async fn create_task(
    handler: web::Data<TaskHandler>,
    payload: web::Json<CreateTaskRequest>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&payload.project_id) {
        Ok(project_id) => {
            let due_date = if let Some(due_date_str) = &payload.due_date {
                match DateTime::parse_from_rfc3339(due_date_str) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
                        "error": "Invalid due_date format. Use RFC3339 format."
                    })))
                }
            } else {
                None
            };

            match handler.service.create_task(
                project_id,
                payload.title.clone(),
                payload.description.clone(),
                payload.priority.clone(),
                payload.assigned_to.clone(),
                due_date,
                payload.created_by.clone(),
            ).await {
                Ok(task) => Ok(HttpResponse::Created().json(task)),
                Err(err) => {
                    tracing::error!("Error creating task: {:?}", err);
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

pub async fn update_task_status(
    handler: web::Data<TaskHandler>,
    query: web::Query<TaskQuery>,
    payload: web::Json<UpdateTaskStatusRequest>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&query.id) {
        Ok(id) => {
            match handler.service.update_task_status(id, payload.status.clone()).await {
                Ok(task) => Ok(HttpResponse::Ok().json(task)),
                Err(AppError::NotFound) => Ok(HttpResponse::NotFound().json(json!({
                    "error": "Task not found"
                }))),
                Err(err) => {
                    tracing::error!("Error updating task status: {:?}", err);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Internal server error"
                    })))
                }
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid task ID format"
        })))
    }
}

pub async fn delete_task(
    handler: web::Data<TaskHandler>,
    query: web::Query<TaskQuery>,
) -> Result<HttpResponse> {
    match Uuid::parse_str(&query.id) {
        Ok(id) => {
            match handler.service.delete_task(id).await {
                Ok(()) => Ok(HttpResponse::NoContent().finish()),
                Err(AppError::NotFound) => Ok(HttpResponse::NotFound().json(json!({
                    "error": "Task not found"
                }))),
                Err(err) => {
                    tracing::error!("Error deleting task: {:?}", err);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": "Internal server error"
                    })))
                }
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid task ID format"
        })))
    }
}

#[derive(serde::Deserialize)]
pub struct TaskQuery {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct ProjectTasksQuery {
    pub project_id: String,
}

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub assigned_to: Option<String>,
    pub due_date: Option<String>, // RFC3339 format
    pub created_by: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
}
