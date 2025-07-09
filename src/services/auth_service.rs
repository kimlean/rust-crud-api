use crate::models::auth_model::*;
use crate::services::database::DatabasePool;
use crate::utils::jwt::create_jwt;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct AuthService {
    db: DatabasePool,
}

impl AuthService {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }

    pub async fn register_user(&self, request: RegisterRequest) -> Result<AuthResponse> {
        // Hash the password
        let password_hash = hash(&request.password, DEFAULT_COST)?;

        // Call the stored procedure
        let query = "SELECT sp_register_user($1, $2, $3) as user_id";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &request.username,
            &request.email,
            &password_hash,
        ];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let user_id: i32 = row.get("user_id");
                let token = create_jwt(user_id)?;
                
                Ok(AuthResponse {
                    user_id,
                    username: request.username,
                    email: request.email,
                    token,
                })
            }
            None => Err(anyhow::anyhow!("Failed to create user")),
        }
    }

    pub async fn login_user(&self, request: LoginRequest) -> Result<AuthResponse> {
        // Call the stored procedure
        let query = "SELECT * FROM sp_login_user($1)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&request.email];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let user_id: i32 = row.get("id");
                let username: String = row.get("username");
                let email: String = row.get("email");
                let password_hash: String = row.get("passwordhash");

                // Verify password
                if verify(&request.password, &password_hash)? {
                    let token = create_jwt(user_id)?;
                    
                    Ok(AuthResponse {
                        user_id,
                        username,
                        email,
                        token,
                    })
                } else {
                    Err(anyhow::anyhow!("Invalid credentials"))
                }
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }
}