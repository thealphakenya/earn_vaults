/*
Template Overview

Earn Vaults is a robust, high-performance API built with Rust and powered by the Actix Web framework. Designed for modern web applications, Earn Vaults provides an efficient solution for managing user data, processing transactions, and integrating artificial intelligence for automated diagnostics and fixes. With a modular architecture, the project splits core functionalities into several components—authentication, task management, AI integration, administrative controls, and database operations—ensuring that each part of the application is maintainable and scalable.

At its core, Earn Vaults uses SQLite (via rusqlite) to store and manage user data and transaction histories. The use of SQLite with bundled support means that the database is lightweight and easily deployable, making it a great choice for projects where low resource consumption is key. Furthermore, environment variables are loaded securely using dotenvy, which ensures that sensitive configuration details like the database URL, API keys, and tokens are not hard-coded but managed externally. This makes the project secure and flexible across different deployment environments.

The project also leverages asynchronous programming with Tokio, which keeps the API responsive even under heavy load by efficiently managing multiple concurrent operations. A key feature is its AI-powered module that integrates with external AI services. This module not only helps in diagnosing issues and suggesting auto-fixes but also provides an administrative interface for real-time interaction and troubleshooting. With built-in webhook handling, the application can easily integrate with other services, allowing for automated triggers and notifications.

For deployment, Earn Vaults is optimized to run on Railway. Its Dockerfile is crafted using a slim Rust image for building and a minimal Debian image for the final deployment, ensuring low memory usage and fast startup times. This architecture, combined with Railway’s robust infrastructure, provides an ideal platform for scaling your application with ease.

Earn Vaults is perfect for developers looking for a secure, scalable API solution that harnesses the power of Rust and AI, offering rapid deployment, automatic backups, and seamless integration with modern web services.
*/

use reqwest;
use std::env;
use tokio;
use log::{info, error};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Load environment variables from a .env file, if available.
    dotenvy::dotenv().ok();

    // Initialize logging with a default filter set to "error" for production.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Retrieve the internal Railway URL from the environment or use a default.
    let railway_internal_url = env::var("RAILWAY_INTERNAL_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    info!("Starting Earn Vaults API...");
    info!("Using internal service URL: {}", railway_internal_url);

    // Make an asynchronous HTTP GET request using the configurable URL.
    match reqwest::get(&railway_internal_url).await {
        Ok(response) => {
            info!("Successfully fetched URL: {}", railway_internal_url);
            // Process the response text.
            match response.text().await {
                Ok(text) => info!("Response Text: {}", text),
                Err(e) => error!("Failed to read response text: {}", e),
            }
        },
        Err(e) => {
            error!("Error fetching URL {}: {}", railway_internal_url, e);
        }
    }
}