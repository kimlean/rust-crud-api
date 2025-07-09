use crate::models::users_model::UserResponse;
use crate::services::database::DatabasePool;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct UserService {
    db: DatabasePool,
}

impl UserService {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<UserResponse>> {
        let query = "SELECT * FROM sp_get_user_by_id($1)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&user_id];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let id: i32 = row.get("id");
                let username: String = row.get("username");
                let email: String = row.get("email");
                let created_at: DateTime<Utc> = row.get("createdat");

                Ok(Some(UserResponse {
                    id,
                    username,
                    email,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }
}