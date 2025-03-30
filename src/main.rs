use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest}; use std::sync::{Arc, Mutex}; use tokio::sync::Mutex as TokioMutex; use tokio::time::{interval, Duration}; use std::fs; use chrono::Utc; use std::path::Path; use rusqlite::Connection; use std::env; use dotenvy::dotenv; use reqwest;

mod auth; mod tasks; mod ai; mod admin; mod db;

// Authentication function async fn authenticate(req: HttpRequest) -> bool { if let Some(auth_header) = req.headers().get("Authorization") { if auth_header == "Bearer your-secure-token" { return true; } } false }

// Webhook handler async fn webhook_handler(body: String) -> impl Responder { println!("Received Webhook: {}", body); HttpResponse::Ok().body("Webhook received") }

// Health check endpoint async fn health_check() -> impl Responder { HttpResponse::Ok().body("OK") }

// Fetch secret from Railway internal API async fn fetch_secret() -> impl Responder { let railway_internal_url = "http://api.railway.internal:3000/secret";

match reqwest::get(railway_internal_url).await {
    Ok(response) => {
        match response.text().await {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(_) => HttpResponse::InternalServerError().body("Failed to read response"),
        }
    }
    Err(_) => HttpResponse::InternalServerError().body("Request to Railway API failed"),
}

}

#[actix_web::main] async fn main() -> std::io::Result<()> { // Load environment variables dotenv().ok(); let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); let api_key = env::var("API_KEY").unwrap_or_else(|| "default-api-key".to_string()); let port = env::var("PORT").unwrap_or_else(|| "8080".to_string()).parse::<u16>().expect("Invalid PORT value");

println!("Connected to database at {}", database_url);
println!("Using API key: {}", api_key);
println!("Server running on port: {}", port);

// Initialize database
let db_conn = db::init_database().expect("Failed to initialize DB");
let db_conn = Arc::new(Mutex::new(db_conn));

let ai_manager = Arc::new(TokioMutex::new(ai::AIManager::new(&api_key)));
let withdrawals_enabled = Arc::new(Mutex::new(true));

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
        .route("/webhook", web::post().to(webhook_handler))
        .route("/health", web::get().to(health_check)) // Health check route
        .route("/fetch-secret", web::get().to(fetch_secret)) // Fetch secret route
})
.bind(("0.0.0.0", port))?
.run()
.await

}

async fn home() -> impl Responder { HttpResponse::Ok().body("Earn Vault API Running") }

// Backup Database fn backup_database(db: &Connection) { let backup_dir = "./backups/";

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

