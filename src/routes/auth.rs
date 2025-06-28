use actix_web::{
    Responder,
    http::StatusCode,
    post,
    web::Json,
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use sqlx::Row;

use crate::{
    routes::{UserDetails, LoginRequest, LoginResponse, logging},
    database::database_connection,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub message: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
}

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

#[post("/auth/register")]
async fn register(user_data: Json<RegisterRequest>) -> HttpResponse {
    logging("POST: /auth/register");
    
    let pool = match database_connection().await {
        Ok(pool) => pool,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database connection failed"
            }));
        }
    };

    // Check if user already exists
    let existing_user = sqlx::query("SELECT user_id FROM users WHERE email = ?")
        .bind(&user_data.email)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(_)) = existing_user {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "User with this email already exists"
        }));
    }

    // Create new user
    let mut new_user = UserDetails::new_with_password(
        user_data.full_name.clone(),
        user_data.email.clone(),
        user_data.password.clone(),
    );

    // Hash password
    if let Err(_) = new_user.hash_password().await {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to hash password"
        }));
    }

    let user_id = new_user.user_id.as_ref().unwrap();

    // Insert user into database
    let result = sqlx::query(
        "INSERT INTO users (user_id, full_name, email, password_hash) VALUES (?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(&new_user.full_name)
    .bind(&new_user.email)
    .bind(&new_user.password)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let response = RegisterResponse {
                message: "User registered successfully".to_string(),
                user_id: user_id.clone(),
            };
            HttpResponse::Created().json(response)
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

#[post("/auth/login")]
async fn login(login_data: Json<LoginRequest>) -> HttpResponse {
    logging("POST: /auth/login");
    
    let pool = match database_connection().await {
        Ok(pool) => pool,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database connection failed"
            }));
        }
    };

    // Find user by email
    let user_row = sqlx::query(
        "SELECT user_id, full_name, email, password_hash FROM users WHERE email = ?"
    )
    .bind(&login_data.email)
    .fetch_optional(&pool)
    .await;

    let user_row = match user_row {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid email or password"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }));
        }
    };

    let user_id: String = user_row.get("user_id");
    let full_name: String = user_row.get("full_name");
    let email: String = user_row.get("email");
    let password_hash: String = user_row.get("password_hash");

    // Verify password
    let is_valid = bcrypt::verify(&login_data.password, &password_hash);
    if let Ok(false) = is_valid {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        }));
    }

    // Generate JWT token
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.clone(),
        email: email.clone(),
        exp: expiration,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate token"
            }));
        }
    };

    let user = UserDetails {
        user_id: Some(user_id),
        full_name,
        email,
        password: String::new(), // Don't send password in response
    };

    let response = LoginResponse { token, user };
    HttpResponse::Ok().json(response)
} 