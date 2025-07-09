use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{auth_handler, notes_handler, users_handler};
use crate::services::database::DatabasePool;
use crate::utils::auth_middleware::auth_middleware;

pub fn create_routes(db_pool: DatabasePool) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .with_state(db_pool.clone());

    let protected_routes = Router::new()
        .route("/users/{id}", get(users_handler::get_user_by_id))
        .route("/notes", post(notes_handler::create_note))
        .route("/notes", get(notes_handler::get_user_notes))
        .route("/notes/search", get(notes_handler::search_notes))
        .route("/notes/{id}", get(notes_handler::get_note_by_id))
        .route("/notes/{id}", put(notes_handler::update_note))
        .route("/notes/{id}", delete(notes_handler::delete_note))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(db_pool);

    Router::new()
        .nest("/auth", auth_routes)
        .merge(protected_routes)
}