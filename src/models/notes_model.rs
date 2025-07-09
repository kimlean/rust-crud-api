use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateNoteRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoteResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchRequest {
    pub search_term: Option<String>,
}