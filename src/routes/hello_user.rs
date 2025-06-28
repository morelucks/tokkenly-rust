use actix_web::{
    Responder, get,
    http::StatusCode,
    web::{Json, Path},
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

#[get("/hello/{full_name}/{email}")]
pub async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let (full_name, email) = params.into_inner();
    let response = UserDetails::new(full_name, email);
    (Json(response), StatusCode::OK)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDetails {
    pub user_id: Option<String>,
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDetails,
}

impl UserDetails {
    fn new(full_name: String, email: String) -> Self {
        Self {
            user_id: Some(Uuid::new_v4().to_string()),
            full_name: full_name,
            email: email,
            password: String::new(),
        }
    }

    pub fn new_with_password(full_name: String, email: String, password: String) -> Self {
        Self {
            user_id: Some(Uuid::new_v4().to_string()),
            full_name: full_name,
            email: email,
            password: password,
        }
    }

    pub async fn hash_password(&mut self) -> Result<(), bcrypt::BcryptError> {
        self.password = hash(self.password.as_bytes(), DEFAULT_COST)?;
        Ok(())
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password)
    }
}
