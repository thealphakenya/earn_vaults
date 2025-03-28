use actix_web::{web, HttpResponse, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone)]
struct User {
    id: i32,
    username: String,
    balance: f32,
    is_online: bool,
}

struct AdminState {
    db: Mutex<Connection>,
}

#[derive(Deserialize)]
struct AdminLogin {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

// Admin login with database validation
pub async fn admin_login(credentials: web::Json<AdminLogin>, data: web::Data<Arc<AdminState>>) -> impl Responder {
    let conn = data.db.lock().unwrap();

    let is_valid_admin = credentials.email == "rovicviccy@gmail.com" && credentials.password == "Victor9798!";

    if is_valid_admin {
        HttpResponse::Ok().json(ApiResponse {
            message: "Welcome, Admin!".to_string(),
            status: "success".to_string(),
        })
    } else {
        HttpResponse::Unauthorized().json(ApiResponse {
            message: "Invalid credentials".to_string(),
            status: "error".to_string(),
        })
    }
}

// Fetch dashboard statistics from the database
pub async fn dashboard(data: web::Data<Arc<AdminState>>) -> impl Responder {
    let conn = data.db.lock().unwrap();

    let total_users: i32 = conn
        .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
        .unwrap_or(0);

    let online_users: i32 = conn
        .query_row("SELECT COUNT(*) FROM users WHERE is_online = 1", [], |row| row.get(0))
        .unwrap_or(0);

    let total_balance: f32 = conn
        .query_row("SELECT SUM(balance) FROM users", [], |row| row.get(0))
        .unwrap_or(0.0);

    let dashboard_info = format!(
        "Users: {}, Online: {}, Total Balance: Ksh {:.2}",
        total_users, online_users, total_balance
    );

    HttpResponse::Ok().json(ApiResponse {
        message: dashboard_info,
        status: "success".to_string(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig, db_conn: Connection) {
    // Ensure the users table exists
    db_conn
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE,
                balance REAL DEFAULT 0.0,
                is_online BOOLEAN DEFAULT 0
            )",
            [],
        )
        .expect("Failed to create users table");

    let state = web::Data::new(Arc::new(AdminState {
        db: Mutex::new(db_conn),
    }));

    cfg.app_data(state.clone())
        .service(web::resource("/admin/login").route(web::post().to(admin_login)))
        .service(web::resource("/admin/dashboard").route(web::get().to(dashboard)));
}