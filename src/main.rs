use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{interval, Duration};
use std::fs;
use chrono::Utc;
use std::path::Path;
use reqwest;
use std::env;
use log::{info, error};
use std::{thread, time};
use dotenvy::dotenv;
use actix_web::middleware::Logger;

mod auth;
mod tasks;
mod ai;
mod admin;
mod db;

extern "C" {
    fn hello_world(); // Declare the C++ function
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> std::io::Result<()> {
    unsafe {
        hello_world(); // Call the C++ function
    }

    // Load environment variables from a .env file, if available.
    dotenv().ok();

    // Initialize logging with a default filter set to "error" for production.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Retrieve the internal service URL from the environment.
    // If not set, default to "http://localhost:8080" (make sure this matches your server port).
    let internal_service_url = env::var("RAILWAY_INTERNAL_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    info!("Starting Earn Vaults API...");
    info!("Using internal service URL: {}", internal_service_url);

    // Optional: wait a few seconds to ensure the server is up before making the HTTP request.
    let delay = time::Duration::from_secs(5);
    thread::sleep(delay);

    // Make an asynchronous HTTP GET request using the configurable URL.
    match reqwest::get(&internal_service_url).await {
        Ok(response) => {
            info!("Successfully fetched URL: {}", internal_service_url);
            // Process the response text.
            match response.text().await {
                Ok(text) => info!("Response Text: {}", text),
                Err(e) => error!("Failed to read response text: {}", e),
            }
        },
        Err(e) => {
            error!("Error fetching URL {}: {}", internal_service_url, e);
        }
    }

    // Start backup system in background
    let app_data = db::init_database().expect("Failed to initialize DB");
    let ai_manager = Arc::new(TokioMutex::new(ai::AIManager::new(&env::var("API_KEY").expect("API_KEY must be set"))));
    let withdrawals_enabled = Arc::new(Mutex::new(true)); // Withdrawals enabled by default

    // Start backup system in background
    let db_clone = app_data.clone();
    tokio::spawn(async move {
        let mut backup_timer = interval(Duration::from_secs(3600)); // Run every 1 hour
        loop {
            backup_timer.tick().await;
            backup_database(&db_clone);
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Optional: Add logging middleware
            .app_data(web::Data::new(app_data.clone()))
            .app_data(web::Data::new(ai_manager.clone()))
            .app_data(web::Data::new(withdrawals_enabled.clone()))
            .configure(auth::config)
            .configure(tasks::config)
            .configure(ai::config)
            .configure(admin::config)
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
fn backup_database(_db: &db::Database) {
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

async fn authenticate(req: HttpRequest) -> bool {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if auth_header == "Bearer your-secure-token" {
            return true;
        }
    }
    false
}