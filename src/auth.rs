use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

pub async fn login(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(format!("Welcome, {}", user.username))
}

pub async fn signup(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json("User signed up successfully")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)))
       .service(web::resource("/signup").route(web::post().to(signup)));
}