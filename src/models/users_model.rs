use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
}