use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Clone)]
struct User {
    username: String,
    balance: f32,
    is_online: bool,
}

struct AdminState {
    users: Mutex<Vec<User>>,
}

#[derive(Deserialize)]
struct AdminLogin {
    email: String,
    password: String,
}

pub async fn admin_login(credentials: web::Json<AdminLogin>) -> impl Responder {
    if credentials.email == "rovicviccy@gmail.com" && credentials.password == "Victor9798!" {
        HttpResponse::Ok().json("Welcome, Admin!")
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

pub async fn dashboard(data: web::Data<AdminState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let total_users = users.len();
    let online_users = users.iter().filter(|user| user.is_online).count();
    let total_balance: f32 = users.iter().map(|user| user.balance).sum();

    let dashboard_info = format!(
        "Users: {}, Online: {}, Total Balance: Ksh {:.2}",
        total_users, online_users, total_balance
    );

    HttpResponse::Ok().json(dashboard_info)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let initial_users = vec![
        User { username: "JohnDoe".to_string(), balance: 500.0, is_online: true },
        User { username: "Alice".to_string(), balance: 300.0, is_online: false },
    ];

    let state = web::Data::new(AdminState {
        users: Mutex::new(initial_users),
    });

    cfg.app_data(state)
       .service(web::resource("/admin/login").route(web::post().to(admin_login)))
       .service(web::resource("/admin/dashboard").route(web::get().to(dashboard)));
}mod auth;
mod tasks;
mod ai;
mod admin;
mod db;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = db::init_database().expect("Failed to initialize DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .configure(auth::config)
            .configure(tasks::config)
            .configure(ai::config)
            .configure(admin::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}