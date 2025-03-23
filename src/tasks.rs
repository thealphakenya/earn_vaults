use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    name: String,
    description: String,
    reward: f32,
    status: String,  // "pending", "in_progress", "completed"
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

pub async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

pub async fn complete_task(task_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    
    for task in tasks.iter_mut() {
        if task.id == *task_id {
            task.status = "completed".to_string();
            return HttpResponse::Ok().json(format!("Task {} completed!", task.id));
        }
    }
    
    HttpResponse::NotFound().json("Task not found")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let initial_tasks = vec![
        Task { id: 1, name: "Survey Task".to_string(), description: "Complete a quick survey".to_string(), reward: 50.0, status: "pending".to_string() },
        Task { id: 2, name: "Watch Video".to_string(), description: "Watch a short advertisement".to_string(), reward: 30.0, status: "pending".to_string() },
    ];

    let state = web::Data::new(AppState {
        tasks: Mutex::new(initial_tasks),
    });

    cfg.app_data(state)
       .service(web::resource("/tasks").route(web::get().to(list_tasks)))
       .service(web::resource("/tasks/{task_id}/complete").route(web::post().to(complete_task)));
}