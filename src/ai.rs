use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Deserialize)]
struct AIQuery {
    query: String,
}

#[derive(Serialize)]
struct AIResponse {
    message: String,
    status: String,
}

pub struct AIManager {
    api_key: String,
}

impl AIManager {
    pub fn new(api_key: &str) -> Self {
        AIManager {
            api_key: api_key.to_string(),
        }
    }

    pub async fn ask_ai(&self, query: &str) -> Result<String, String> {
        match query.to_lowercase().as_str() {
            "diagnose issues" => Ok("System diagnosis initiated. Checking logs and performance...".to_string()),
            "how many users are online?" => Ok("Live user count is being fetched...".to_string()),
            "optimize performance" => Ok("Performance optimization in progress...".to_string()),
            _ => Ok("I'm here to assist! Ask about system status, user statistics, or troubleshooting.".to_string()),
        }
    }
}

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

pub fn config(cfg: &mut web::ServiceConfig) {
    let ai_manager = Arc::new(Mutex::new(AIManager::new("your-openai-api-key")));
    cfg.app_data(web::Data::new(ai_manager))
        .service(web::resource("/ai").route(web::post().to(ai_assistant)));
}