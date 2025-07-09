use crate::models::auth_model::ApiError;
use crate::models::notes_model::*;
use crate::services::database::DatabasePool;
use crate::services::note_service::NoteService;
use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    response::Json,
};
use validator::Validate;
use tracing::{info, error};

/// Create a new note
#[utoipa::path(
    post,
    path = "/api/v1/notes",
    request_body = CreateNoteRequest,
    responses(
        (status = 201, description = "Note created successfully", body = NoteResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_note(
    State(db_pool): State<DatabasePool>,
    Extension(user_id): Extension<i32>,
    Json(request): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>), (StatusCode, Json<ApiError>)> {
    info!("Attempting to create a new note for user_id: {}", user_id);
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

    let note_service = NoteService::new(db_pool);
    
    match note_service.create_note(request, user_id).await {
        Ok(note) => {
            info!("Successfully created note with id: {}", note.id);
            Ok((StatusCode::CREATED, Json(note)))
        },
        Err(err) => {
            error!("Failed to create note for user_id: {}: {}", user_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Note Creation Failed".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}

/// Get all notes for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/notes",
    responses(
        (status = 200, description = "Notes retrieved successfully", body = [NoteResponse]),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_notes(
    State(db_pool): State<DatabasePool>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<NoteResponse>>, (StatusCode, Json<ApiError>)> {
    info!("Attempting to retrieve notes for user_id: {}", user_id);
    let note_service = NoteService::new(db_pool);
    
    match note_service.get_user_notes(user_id).await {
        Ok(notes) => {
            info!("Successfully retrieved {} notes for user_id: {}", notes.len(), user_id);
            Ok(Json(notes))
        },
        Err(err) => {
            error!("Failed to retrieve notes for user_id: {}: {}", user_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Failed to retrieve notes".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}

/// Get a specific note by ID
#[utoipa::path(
    get,
    path = "/api/v1/notes/{id}",
    params(
        ("id" = i32, Path, description = "Note ID")
    ),
    responses(
        (status = 200, description = "Note found", body = NoteResponse),
        (status = 404, description = "Note not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_note_by_id(
    State(db_pool): State<DatabasePool>,
    Path(note_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<NoteResponse>, (StatusCode, Json<ApiError>)> {
    info!("Attempting to retrieve note with id: {} for user_id: {}", note_id, user_id);
    let note_service = NoteService::new(db_pool);
    
    match note_service.get_note_by_id(note_id, user_id).await {
        Ok(Some(note)) => {
            info!("Successfully retrieved note with id: {}", note_id);
            Ok(Json(note))
        },
        Ok(None) => {
            error!("Note with id: {} not found for user_id: {}", note_id, user_id);
            Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "Note Not Found".to_string(),
                message: "Note with the specified ID was not found".to_string(),
            }),
        ))
        },
        Err(err) => {
            error!("Failed to retrieve note with id: {}: {}", note_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Internal Server Error".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}

/// Update a note
#[utoipa::path(
    put,
    path = "/api/v1/notes/{id}",
    params(
        ("id" = i32, Path, description = "Note ID")
    ),
    request_body = UpdateNoteRequest,
    responses(
        (status = 200, description = "Note updated successfully", body = NoteResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 404, description = "Note not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_note(
    State(db_pool): State<DatabasePool>,
    Path(note_id): Path<i32>,
    Extension(user_id): Extension<i32>,
    Json(request): Json<UpdateNoteRequest>,
) -> Result<Json<NoteResponse>, (StatusCode, Json<ApiError>)> {
    info!("Attempting to update note with id: {} for user_id: {}", note_id, user_id);
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

    let note_service = NoteService::new(db_pool);
    
    match note_service.update_note(note_id, request, user_id).await {
        Ok(Some(note)) => {
            info!("Successfully updated note with id: {}", note_id);
            Ok(Json(note))
        },
        Ok(None) => {
            error!("Note with id: {} not found for user_id: {}", note_id, user_id);
            Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "Note Not Found".to_string(),
                message: "Note with the specified ID was not found".to_string(),
            }),
        ))
        },
        Err(err) => {
            error!("Failed to update note with id: {}: {}", note_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Note Update Failed".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}

/// Delete a note
#[utoipa::path(
    delete,
    path = "/api/v1/notes/{id}",
    params(
        ("id" = i32, Path, description = "Note ID")
    ),
    responses(
        (status = 204, description = "Note deleted successfully"),
        (status = 404, description = "Note not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_note(
    State(db_pool): State<DatabasePool>,
    Path(note_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    info!("Attempting to delete note with id: {} for user_id: {}", note_id, user_id);
    let note_service = NoteService::new(db_pool);
    
    match note_service.delete_note(note_id, user_id).await {
        Ok(true) => {
            info!("Successfully deleted note with id: {}", note_id);
            Ok(StatusCode::NO_CONTENT)
        },
        Ok(false) => {
            error!("Note with id: {} not found for user_id: {}", note_id, user_id);
            Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "Note Not Found".to_string(),
                message: "Note with the specified ID was not found".to_string(),
            }),
        ))
        },
        Err(err) => {
            error!("Failed to delete note with id: {}: {}", note_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Note Deletion Failed".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}

/// Search notes
#[utoipa::path(
    get,
    path = "/api/v1/notes/search",
    params(
        ("search_term" = Option<String>, Query, description = "Search term to filter notes")
    ),
    responses(
        (status = 200, description = "Notes retrieved successfully", body = [NoteResponse]),
        (status = 401, description = "Unauthorized", body = ApiError)
    ),
    tag = "notes",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn search_notes(
    State(db_pool): State<DatabasePool>,
    Extension(user_id): Extension<i32>,
    Query(search_request): Query<SearchRequest>,
) -> Result<Json<Vec<NoteResponse>>, (StatusCode, Json<ApiError>)> {
    info!("Attempting to search notes with term: {:?} for user_id: {}", search_request.search_term, user_id);
    let note_service = NoteService::new(db_pool);
    
    match note_service.search_notes(user_id, search_request.search_term).await {
        Ok(notes) => {
            info!("Successfully found {} notes for user_id: {}", notes.len(), user_id);
            Ok(Json(notes))
        },
        Err(err) => {
            error!("Failed to search notes for user_id: {}: {}", user_id, err);
            Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Search Failed".to_string(),
                message: err.to_string(),
            }),
        ))
        },
    }
}