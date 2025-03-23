use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AIQuery {
    query: String,
}

pub async fn ai_assistant(query: web::Json<AIQuery>) -> impl Responder {
    let response = match query.query.to_lowercase().as_str() {
        "diagnose issues" => "Checking system performance and logs...",
        "how many users are online?" => "Fetching live user count...",
        "optimize performance" => "Adjusting settings for optimal speed...",
        _ => "I am here to assist. Ask me about system status, user stats, or troubleshooting!",
    };

    HttpResponse::Ok().json(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ai").route(web::post().to(ai_assistant)));
}