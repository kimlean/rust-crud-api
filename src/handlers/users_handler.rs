use crate::models::auth_model::ApiError;
use crate::models::users_model::UserResponse;
use crate::services::database::DatabasePool;
use crate::services::user_service::UserService;
use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    response::Json,
};

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_by_id(
    State(db_pool): State<DatabasePool>,
    Path(user_id): Path<i32>,
    Extension(current_user_id): Extension<i32>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ApiError>)> {
    // Check if user is requesting their own information
    if current_user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError {
                error: "Forbidden".to_string(),
                message: "You can only access your own user information".to_string(),
            }),
        ));
    }

    let user_service = UserService::new(db_pool);
    
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "User Not Found".to_string(),
                message: "User with the specified ID was not found".to_string(),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Internal Server Error".to_string(),
                message: err.to_string(),
            }),
        )),
    }
}