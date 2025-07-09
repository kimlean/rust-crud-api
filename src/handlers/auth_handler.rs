use crate::models::auth_model::*;
use crate::services::auth_service::AuthService;
use crate::services::database::DatabasePool;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use validator::Validate;
use tracing::{info, error};

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 409, description = "User already exists", body = ApiError)
    ),
    tag = "auth"
)]
pub async fn register(
    State(db_pool): State<DatabasePool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    info!("Attempting to register a new user with email: {}", request.email);
    // Validate request
    if let Err(errors) = request.validate() {
        error!("Validation failed: {}", errors);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "Validation Error".to_string(),
                message: format!("Validation failed: {}", errors),
            }),
        ));
    }

    let auth_service = AuthService::new(db_pool);
    
    match auth_service.register_user(request).await {
        Ok(response) => {
            info!("Successfully registered user with email: {}", response.email);
            Ok(Json(response))
        },
        Err(err) => {
            error!("Failed to register user: {}", err);
            let status_code = if err.to_string().contains("duplicate") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            
            Err((
                status_code,
                Json(ApiError {
                    error: "Registration Failed".to_string(),
                    message: err.to_string(),
                }),
            ))
        }
    }
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 401, description = "Invalid credentials", body = ApiError)
    ),
    tag = "auth"
)]
pub async fn login(
    State(db_pool): State<DatabasePool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    info!("Login request received for email: {}", request.email);
    // Validate request
    if let Err(errors) = request.validate() {
        error!("Validation failed: {}", errors);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "Validation Error".to_string(),
                message: format!("Validation failed: {}", errors),
            }),
        ));
    }

    let auth_service = AuthService::new(db_pool);
    
    match auth_service.login_user(request).await {
        Ok(response) => {
            info!("Successfully logged in user with email: {}", response.email);
            Ok(Json(response))
        },
        Err(err) => {
            error!("Failed to login user: {}", err);
            let status_code = if err.to_string().contains("Invalid credentials") || err.to_string().contains("User not found") {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            
            Err((
                status_code,
                Json(ApiError {
                    error: "Login Failed".to_string(),
                    message: err.to_string(),
                }),
            ))
        }
    }
}