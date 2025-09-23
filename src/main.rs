use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod project;
mod task;

use project::{Project, ProjectStatus};
use task::{Task, TaskStatus, TaskPriority};

fn main() {
    println!("🚀 ILM - Issue & Project Management System");
    println!("==========================================\n");

    // Create sample users
    let owner_id = Uuid::new_v4();
    let assignee_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();
    
    // Create a new project
    let mut project = Project::new(
        "Web Application Redesign".to_string(),
        Some("Complete redesign of the company website with modern UI/UX".to_string()),
        owner_id,
        owner_id, // created_by is the owner
    );    println!("📁 Created Project:");
    println!("   {}", project);
    println!("   ID: {}", project.id);
    println!("   Owner: {}", project.owner_id);
    println!("   Created: {} by {}", project.created_at_formatted(), project.created_by);
    println!("   Last Updated: {} by {}\n", project.updated_at_formatted(), project.updated_by);

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

    // Update task statuses and priorities
    tasks[0].update_priority(TaskPriority::High, owner_id);
    tasks[1].update_status(TaskStatus::InProgress, assignee_id);
    tasks[2].update_priority(TaskPriority::Low, admin_id);

    println!("📋 Created Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("   {}. {}", i + 1, task);
        if task.is_overdue() {
            println!("      ⚠️  This task is overdue!");
        }
        if let Some(due_date_str) = task.due_date_formatted() {
            println!("      📅 Due: {}", due_date_str);
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
    project.update_status(ProjectStatus::Active, owner_id);
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
    
    println!("\n⏱️  Millisecond Precision Demo:");
    println!("   Project age: {} ms ({} seconds)", project.age_in_milliseconds(), project.age_in_seconds());
    for (i, task) in tasks.iter().enumerate() {
        println!("   Task {}: {} ms old", i + 1, task.age_in_milliseconds());
        if let Some(time_left_ms) = task.time_until_due_milliseconds() {
            if time_left_ms > 0 {
                println!("             {} ms until due ({} seconds)", time_left_ms, time_left_ms / 1000);
            } else {
                println!("             {} ms overdue ({} seconds)", -time_left_ms, -time_left_ms / 1000);
            }
        }
    }
}
