extern "C" {
    fn hello_world(); // Declare the C++ function
}

use reqwest;
use std::env;
use tokio;
use log::{info, error};
use std::{thread, time};

/// Earn Vaults: High-Performance API
///
/// # Overview
/// Earn Vaults is a robust, high-performance API built with Rust and powered by the Actix Web framework.
/// Designed for modern web applications, it provides efficient solutions for managing user data,
/// processing transactions, and integrating AI for automated diagnostics and fixes.
///
/// # Architecture
/// - **Authentication Module**: Secure user authentication with JWT.
/// - **Task Management**: Handles various asynchronous jobs efficiently.
/// - **AI Integration**: Provides automated diagnostics and fixes.
/// - **Database Operations**: Uses SQLite (`rusqlite`) with bundled support.
/// - **Environment Management**: Securely loads environment variables with `dotenvy`.
///
/// # Performance
/// - Built using **Tokio** for high concurrency.
/// - Uses **SQLite (rusqlite)** for lightweight, fast transactions.
/// - **AI-powered module** integrated with OpenAI services.
///
/// # Deployment
/// - Optimized for **Railway**.
/// - Uses a **minimal Docker image** for fast deployment.
/// - Supports **automatic backups** and **webhook handling**.

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    unsafe {
        hello_world(); // Call the C++ function
    }

    // Load environment variables from a .env file, if available.
    dotenvy::dotenv().ok();

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
}
