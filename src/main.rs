use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{interval, Duration};
use std::fs;
use chrono::Utc;
use std::path::Path;
use rusqlite::Connection;
use std::env;
use dotenvy::dotenv;

mod auth;
mod tasks;
mod ai;
mod admin;
mod db;

// Authentication function
async fn authenticate(req: HttpRequest) -> bool {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if auth_header == "Bearer your-secure-token" {
            return true;
        }
    }
    false
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default-api-key".to_string());

    println!("Connected to database at {}", database_url);
    println!("Using API key: {}", api_key);

    // Initialize database
    let db_conn = db::init_database().expect("Failed to initialize DB");
    let db_conn = Arc::new(Mutex::new(db_conn)); // Wrap connection in Arc<Mutex<>>

    let ai_manager = Arc::new(TokioMutex::new(ai::AIManager::new(&api_key)));
    let withdrawals_enabled = Arc::new(Mutex::new(true)); // Withdrawals enabled by default

    // Start backup system in the background
    let db_clone = Arc::clone(&db_conn);
    tokio::spawn(async move {
        let mut backup_timer = interval(Duration::from_secs(3600)); // Run every 1 hour
        loop {
            backup_timer.tick().await;
            let db_guard = db_clone.lock().unwrap();
            backup_database(&db_guard);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&db_conn)))
            .app_data(web::Data::new(ai_manager.clone()))
            .app_data(web::Data::new(withdrawals_enabled.clone()))
            .configure(|cfg| auth::config(cfg, Arc::clone(&db_conn)))
            .configure(|cfg| tasks::config(cfg, Arc::clone(&db_conn)))
            .configure(|cfg| ai::config(cfg, ai_manager.clone()))
            .configure(|cfg| admin::config(cfg, Arc::clone(&db_conn)))
            .route("/", web::get().to(home))
            .route("/diagnose", web::post().to(diagnose_issue))
            .route("/auto-fix", web::post().to(auto_fix_issue))
            .route("/admin/ai", web::post().to(admin_ai_interface))
            .route("/admin/toggle_withdrawals", web::post().to(toggle_withdrawals))
            .route("/withdraw", web::post().to(handle_withdrawal))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("Earn Vault API Running")
}

// AI Diagnosis Function
async fn diagnose_issue(
    req: HttpRequest,
    data: web::Data<Arc<TokioMutex<ai::AIManager>>>,
    issue: web::Json<String>,
) -> impl Responder {
    if !authenticate(req).await {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    let ai = data.lock().await;
    match ai.ask_ai(&format!("Diagnose this issue: {}", issue)).await {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::InternalServerError().body("AI diagnosis failed"),
    }
}

// AI Auto-Fix Function
async fn auto_fix_issue(
    req: HttpRequest,
    data: web::Data<Arc<TokioMutex<ai::AIManager>>>,
    issue: web::Json<String>,
) -> impl Responder {
    if !authenticate(req).await {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    let ai = data.lock().await;
    match ai.ask_ai(&format!("Auto-fix this issue: {}", issue)).await {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::InternalServerError().body("AI auto-fix failed"),
    }
}

// Admin AI Interface
async fn admin_ai_interface(
    req: HttpRequest,
    data: web::Data<Arc<TokioMutex<ai::AIManager>>>,
    input: web::Json<String>,
) -> impl Responder {
    if !authenticate(req).await {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    let ai = data.lock().await;
    match ai.ask_ai(&input).await {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::InternalServerError().body("AI interaction failed"),
    }
}

// Admin: Toggle Withdrawals
async fn toggle_withdrawals(
    req: HttpRequest,
    state: web::Data<Arc<Mutex<bool>>>,
) -> impl Responder {
    if !authenticate(req).await {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let mut withdrawals_enabled = state.lock().unwrap();
    *withdrawals_enabled = !*withdrawals_enabled; // Toggle the state

    let status = if *withdrawals_enabled {
        "Withdrawals are now ENABLED."
    } else {
        "Withdrawals are now DISABLED."
    };

    HttpResponse::Ok().body(status)
}

// User Withdrawal Request
async fn handle_withdrawal(
    req: HttpRequest,
    state: web::Data<Arc<Mutex<bool>>>,
) -> impl Responder {
    let withdrawals_enabled = state.lock().unwrap();

    if !*withdrawals_enabled {
        return HttpResponse::BadRequest().body("Withdrawal unavailable at the moment.");
    }

    HttpResponse::Ok().body("Withdrawal request received. Processing...")
}

// Backup Database
fn backup_database(db: &Connection) {
    let backup_dir = "./backups/";

    if !Path::new(backup_dir).exists() {
        fs::create_dir_all(backup_dir).expect("Failed to create backup directory");
    }

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_path = format!("{}/backup_{}.db", backup_dir, timestamp);

    match fs::copy("./earn_vault.db", &backup_path) {
        Ok(_) => println!("Database backed up successfully at {}", backup_path),
        Err(e) => eprintln!("Database backup failed: {}", e),
    }
}