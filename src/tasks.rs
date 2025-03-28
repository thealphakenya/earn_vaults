use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: i32,
    name: String,
    description: String,
    reward: f32,
    status: String, // "pending", "in_progress", "completed"
}

struct AppState {
    db: Mutex<Connection>,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

// List all tasks
pub async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let mut stmt = match conn.prepare("SELECT id, name, description, reward, status FROM tasks") {
        Ok(stmt) => stmt,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                message: "Database query error".to_string(),
                status: "error".to_string(),
            });
        }
    };

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            reward: row.get(3)?,
            status: row.get(4)?,
        })
    });

    match task_iter {
        Ok(tasks) => {
            let task_list: Vec<Task> = tasks.filter_map(Result::ok).collect();
            HttpResponse::Ok().json(task_list)
        }
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            message: "Failed to fetch tasks".to_string(),
            status: "error".to_string(),
        }),
    }
}

// Complete a task
pub async fn complete_task(task_id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let updated = conn.execute(
        "UPDATE tasks SET status = 'completed' WHERE id = ?1",
        [task_id.to_string()],
    );

    match updated {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json(ApiResponse {
            message: format!("Task {} marked as completed!", task_id),
            status: "success".to_string(),
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse {
            message: "Task not found".to_string(),
            status: "error".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            message: "Failed to update task".to_string(),
            status: "error".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig, db_conn: Connection) {
    // Ensure the tasks table exists
    db_conn
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                description TEXT,
                reward REAL,
                status TEXT DEFAULT 'pending'
            )",
            [],
        )
        .expect("Failed to create tasks table");

    let state = web::Data::new(AppState {
        db: Mutex::new(db_conn),
    });

    cfg.app_data(state)
        .service(web::resource("/tasks").route(web::get().to(list_tasks)))
        .service(web::resource("/tasks/{task_id}/complete").route(web::post().to(complete_task)));
}