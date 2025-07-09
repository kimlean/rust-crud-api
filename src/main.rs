use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use services::database::DatabasePool;
use crate::models::{
    auth_model::{ApiError, AuthResponse, LoginRequest, RegisterRequest},
    notes_model::{CreateNoteRequest, NoteResponse, SearchRequest, UpdateNoteRequest},
    users_model::UserResponse,
};
use crate::handlers::{
    auth_handler,
    notes_handler,
    users_handler,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::register,
        auth_handler::login,
        users_handler::get_user_by_id,
        notes_handler::create_note,
        notes_handler::get_user_notes,
        notes_handler::get_note_by_id,
        notes_handler::update_note,
        notes_handler::delete_note,
        notes_handler::search_notes,
    ),
    components(schemas(
        RegisterRequest,
        LoginRequest,
        AuthResponse,
        CreateNoteRequest,
        UpdateNoteRequest,
        NoteResponse,
        SearchRequest,
        UserResponse,
        ApiError,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "notes", description = "Notes management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database pool
    let db_pool = DatabasePool::new().await?;

    // Create the router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", routes::create_routes(db_pool))
        .layer(CorsLayer::permissive());

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}