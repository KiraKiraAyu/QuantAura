use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenPayload {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagePayload {
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrentUserPayload {
    pub user_id: String,
    pub email: String,
}
