use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use argon2::{self, Config};
use rand::Rng;

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    message: String,
    status: String,
}

pub async fn login(
    user: web::Json<UserCredentials>,
    db: web::Data<Connection>,
) -> impl Responder {
    let conn = db.get_ref();

    let mut stmt = match conn.prepare("SELECT password FROM users WHERE username = ?1") {
        Ok(stmt) => stmt,
        Err(_) => {
            return HttpResponse::InternalServerError().json(AuthResponse {
                message: "Database error".to_string(),
                status: "error".to_string(),
            });
        }
    };

    let stored_password: Result<String, _> = stmt.query_row([&user.username], |row| row.get(0));

    match stored_password {
        Ok(hash) => {
            if argon2::verify_encoded(&hash, user.password.as_bytes()).unwrap_or(false) {
                HttpResponse::Ok().json(AuthResponse {
                    message: format!("Welcome, {}", user.username),
                    status: "success".to_string(),
                })
            } else {
                HttpResponse::Unauthorized().json(AuthResponse {
                    message: "Invalid credentials".to_string(),
                    status: "error".to_string(),
                })
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(AuthResponse {
            message: "User not found".to_string(),
            status: "error".to_string(),
        }),
    }
}

pub async fn signup(
    user: web::Json<UserCredentials>,
    db: web::Data<Connection>,
) -> impl Responder {
    let conn = db.get_ref();

    // Generate a random salt
    let salt: [u8; 16] = rand::thread_rng().gen();
    let config = Config::default();
    let hash = argon2::hash_encoded(user.password.as_bytes(), &salt, &config).unwrap();

    let result = conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        [&user.username, &hash],
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(AuthResponse {
            message: "User signed up successfully".to_string(),
            status: "success".to_string(),
        }),
        Err(_) => HttpResponse::BadRequest().json(AuthResponse {
            message: "Username already exists".to_string(),
            status: "error".to_string(),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)))
       .service(web::resource("/signup").route(web::post().to(signup)));
}