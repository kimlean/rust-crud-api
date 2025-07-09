use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}