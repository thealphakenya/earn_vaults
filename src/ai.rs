use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;

/// Represents the structure of the incoming AI query.
#[derive(Deserialize)]
struct AIQuery {
    query: String,
}

/// Represents the structure of the response sent back to the client.
#[derive(Serialize)]
struct AIResponse {
    message: String,
    status: String,
}

/// Manages AI operations by holding an API key.
pub struct AIManager {
    api_key: String,
}

impl AIManager {
    /// Creates a new AIManager with the provided API key.
    pub fn new(api_key: &str) -> Self {
        AIManager {
            api_key: api_key.to_string(),
        }
    }

    /// Processes the query and returns a canned response.
    /// In a real implementation, this method would make HTTP requests to an AI service.
    pub async fn ask_ai(&self, query: &str) -> Result<String, String> {
        // For demonstration, we provide static responses based on the query.
        match query.to_lowercase().as_str() {
            "diagnose issues" => Ok("System diagnosis initiated. Checking logs and performance...".to_string()),
            "how many users are online?" => Ok("Live user count is being fetched...".to_string()),
            "optimize performance" => Ok("Performance optimization in progress...".to_string()),
            _ => Ok("I'm here to assist! Ask about system status, user statistics, or troubleshooting.".to_string()),
        }
    }
}

/// HTTP handler that processes incoming AI queries.
pub async fn ai_assistant(
    query: web::Json<AIQuery>,
    ai_manager: web::Data<Arc<Mutex<AIManager>>>,
) -> impl Responder {
    let ai = ai_manager.lock().await;
    match ai.ask_ai(&query.query).await {
        Ok(response) => HttpResponse::Ok().json(AIResponse {
            message: response,
            status: "success".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(AIResponse {
            message: "AI request failed".to_string(),
            status: "error".to_string(),
        }),
    }
}

/// Configures the Actix Web service, initializing the AIManager with an API key
/// read from the environment variable `OPENAI_API_KEY`. If the variable is not set,
/// a default value is used.
pub fn config(cfg: &mut web::ServiceConfig) {
    // Read the API key from the environment variable; use "default-api-key" if not set.
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "default-api-key".to_string());
    let ai_manager = Arc::new(Mutex::new(AIManager::new(&api_key)));

    cfg.app_data(web::Data::new(ai_manager))
       .service(web::resource("/ai").route(web::post().to(ai_assistant)));
}