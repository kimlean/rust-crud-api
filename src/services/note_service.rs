use crate::models::notes_model::*;
use crate::services::database::DatabasePool;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct NoteService {
    db: DatabasePool,
}

impl NoteService {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }

    pub async fn create_note(&self, request: CreateNoteRequest, user_id: i32) -> Result<NoteResponse> {
        let query = "SELECT * FROM sp_create_or_update_note($1, $2, $3, $4) as note_id";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &request.id,
            &request.title,
            &request.content,
            &user_id,
        ];

        let row = self.db.execute_query_one(query, params).await?;
        println!("Row returned from database: {:?}", row);
        match row {
            Some(row) => {
                let note_id: i32 = row.get("noteid");

                // Get the created note
                self.get_note_by_id(note_id, user_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Failed to retrieve created note"))
            }
            None => Err(anyhow::anyhow!("Failed to create note")),
        }
    }

    pub async fn get_user_notes(&self, user_id: i32) -> Result<Vec<NoteResponse>> {
        let query = "SELECT * FROM sp_get_user_notes($1)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&user_id];

        let rows = self.db.execute_query(query, params).await?;
        
        let notes = rows.into_iter().map(|row| {
            let id: i32 = row.get("id");
            let title: String = row.get("title");
            let content: String = row.get("content");
            let created_at: DateTime<Utc> = row.get("createdat");
            let updated_at: DateTime<Utc> = row.get("updatedat");

            NoteResponse {
                id,
                title,
                content,
                created_at,
                updated_at,
            }
        }).collect();

        Ok(notes)
    }

    pub async fn get_note_by_id(&self, note_id: i32, user_id: i32) -> Result<Option<NoteResponse>> {
        let query = "SELECT * FROM sp_get_note_by_id($1, $2)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&note_id, &user_id];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let id: i32 = row.get("id");
                let title: String = row.get("title");
                let content: String = row.get("content");
                let created_at: DateTime<Utc> = row.get("createdat");
                let updated_at: DateTime<Utc> = row.get("updatedat");

                Ok(Some(NoteResponse {
                    id,
                    title,
                    content,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn update_note(&self, note_id: i32, request: UpdateNoteRequest, user_id: i32) -> Result<Option<NoteResponse>> {
        let query = "SELECT sp_update_note($1, $2, $3, $4) as updated";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &note_id,
            &request.title,
            &request.content,
            &user_id,
        ];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let updated: i32 = row.get("updated");
                if updated == 1 {
                    self.get_note_by_id(note_id, user_id).await
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn delete_note(&self, note_id: i32, user_id: i32) -> Result<bool> {
        let query = "SELECT sp_delete_note($1, $2) as deleted";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&note_id, &user_id];

        let row = self.db.execute_query_one(query, params).await?;
        
        match row {
            Some(row) => {
                let deleted: i32 = row.get("deleted");
                Ok(deleted == 1)
            }
            None => Ok(false),
        }
    }

    pub async fn search_notes(&self, user_id: i32, search_term: Option<String>) -> Result<Vec<NoteResponse>> {
        let query = "SELECT * FROM sp_search_notes($1, $2)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&user_id, &search_term];

        let rows = self.db.execute_query(query, params).await?;
        
        let notes = rows.into_iter().map(|row| {
            let id: i32 = row.get("id");
            let title: String = row.get("title");
            let content: String = row.get("content");
            let created_at: DateTime<Utc> = row.get("createdat");
            let updated_at: DateTime<Utc> = row.get("updatedat");

            NoteResponse {
                id,
                title,
                content,
                created_at,
                updated_at,
            }
        }).collect();

        Ok(notes)
    }
}