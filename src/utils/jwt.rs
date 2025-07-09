use crate::models::auth_model::Claims;
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &str = "your_secret_key_here"; // In production, use environment variable

pub fn create_jwt(user_id: i32) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 60 * 60 * 24; // 24 hours

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?;

    Ok(token)
}

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub fn extract_user_id_from_token(token: &str) -> Result<i32> {
    let claims = validate_jwt(token)?;
    let user_id = claims.sub.parse::<i32>()?;
    Ok(user_id)
}