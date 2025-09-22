use chrono::{Duration, Utc};
use uuid::Uuid;

mod project;
mod task;

use project::{Project, ProjectStatus};
use task::{Task, TaskStatus, TaskPriority};

fn main() {
    println!("🚀 ILM - Issue & Project Management System");
    println!("==========================================\n");

    // Create a sample user (owner)
    let owner_id = Uuid::new_v4();
    let assignee_id = Uuid::new_v4();

    // Create a new project
    let mut project = Project::new(
        "Web Application Redesign".to_string(),
        Some("Complete redesign of the company website with modern UI/UX".to_string()),
        owner_id,
    );

    println!("📁 Created Project:");
    println!("   {}", project);
    println!("   ID: {}", project.id);
    println!("   Owner: {}", project.owner_id);
    println!("   Created: {}\n", project.created_at.format("%Y-%m-%d %H:%M:%S"));

    // Validate the project
    match project.validate() {
        Ok(_) => println!("✅ Project validation passed\n"),
        Err(e) => println!("❌ Project validation failed: {}\n", e),
    }

    // Create some tasks for the project
    let mut tasks = vec![
        Task::new(
            "Design new homepage mockup".to_string(),
            Some("Create wireframes and high-fidelity mockups for the new homepage".to_string()),
            project.id,
            Some(assignee_id),
            Some(Utc::now() + Duration::days(7)),
        ),
        Task::new(
            "Set up development environment".to_string(),
            Some("Configure local development setup with latest framework versions".to_string()),
            project.id,
            Some(assignee_id),
            Some(Utc::now() + Duration::days(3)),
        ),
        Task::new(
            "Research competitor websites".to_string(),
            None,
            project.id,
            None,
            Some(Utc::now() - Duration::days(1)), // This will be overdue
        ),
    ];

    // Update task statuses and priorities
    tasks[0].update_priority(TaskPriority::High);
    tasks[1].update_status(TaskStatus::InProgress);
    tasks[2].update_priority(TaskPriority::Low);

    println!("📋 Created Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("   {}. {}", i + 1, task);
        if task.is_overdue() {
            println!("      ⚠️  This task is overdue!");
        }
        if let Some(due_date) = task.due_date {
            println!("      📅 Due: {}", due_date.format("%Y-%m-%d %H:%M:%S"));
        }
        println!();
    }

    // Validate tasks
    let mut valid_tasks = 0;
    for task in &tasks {
        match task.validate() {
            Ok(_) => valid_tasks += 1,
            Err(e) => println!("❌ Task validation failed: {}", e),
        }
    }
    println!("✅ {}/{} tasks passed validation\n", valid_tasks, tasks.len());

    // Demonstrate project status updates
    project.update_status(ProjectStatus::Active);
    println!("🔄 Updated project status to: {}\n", project.status);

    // Show project and task statistics
    let total_tasks = tasks.len();
    let completed_tasks = tasks.iter().filter(|t| t.is_completed()).count();
    let in_progress_tasks = tasks.iter().filter(|t| t.status == TaskStatus::InProgress).count();
    let overdue_tasks = tasks.iter().filter(|t| t.is_overdue()).count();
    let assigned_tasks = tasks.iter().filter(|t| t.is_assigned()).count();

    println!("📊 Project Statistics:");
    println!("   Total tasks: {}", total_tasks);
    println!("   Completed: {}", completed_tasks);
    println!("   In Progress: {}", in_progress_tasks);
    println!("   Overdue: {}", overdue_tasks);
    println!("   Assigned: {}", assigned_tasks);
    println!("   Completion rate: {:.1}%", (completed_tasks as f64 / total_tasks as f64) * 100.0);
}
