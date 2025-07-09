use crate::utils::jwt::extract_user_id_from_token;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            match extract_user_id_from_token(token) {
                Ok(user_id) => {
                    request.extensions_mut().insert(user_id);
                    Ok(next.run(request).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}