use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod project;
mod task;

use project::Project;
use task::Task;

fn main() {
    println!("🚀 ILM - Issue & Project Management System");
    println!("==========================================\n");

    // Create sample users
    let owner_id = Uuid::new_v4();
    let assignee_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();
    
    // Create a new project
    let project = Project::new(
        "Web Application Redesign".to_string(),
        Some("Complete redesign of the company website with modern UI/UX".to_string()),
        owner_id,
        owner_id, // created_by is the owner
    );
    
    println!("📁 Created Project:");
    println!("   {}", project);
    println!("   ID: {}", project.id);
    println!("   Owner: {}", project.owner_id);

    // Create some tasks for the project
    let mut tasks = vec![
        Task::new(
            "Design new homepage mockup".to_string(),
            Some("Create wireframes and high-fidelity mockups for the new homepage".to_string()),
            project.id,
            Some(assignee_id),
            Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 + 7 * 24 * 3600 * 1000), // 7 days from now
            owner_id, // created_by
        ),
        Task::new(
            "Set up development environment".to_string(),
            Some("Configure local development setup with latest framework versions".to_string()),
            project.id,
            Some(assignee_id),
            Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 + 3 * 24 * 3600 * 1000), // 3 days from now
            owner_id, // created_by
        ),
        Task::new(
            "Research competitor websites".to_string(),
            None,
            project.id,
            None,
            Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 - 24 * 3600 * 1000), // 1 day ago (overdue)
            admin_id, // created_by (different user)
        ),
    ];

    // Modify some tasks to show different statuses and priorities
    use task::{TaskStatus, TaskPriority};
    
    tasks[0].priority = TaskPriority::High;
    tasks[1].status = TaskStatus::InProgress;
    tasks[1].priority = TaskPriority::Medium;
    tasks[2].priority = TaskPriority::Low;

    println!("\n📋 Created Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("   {}. {}", i + 1, task);
    }
}
