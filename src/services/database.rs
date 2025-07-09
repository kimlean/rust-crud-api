use tokio_postgres::{Client, NoTls, Row};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

#[derive(Clone)]
pub struct DatabasePool {
    client: Arc<Mutex<Client>>,
}

impl DatabasePool {
    pub async fn new() -> Result<Self> {
        let connection_string = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "host=127.0.0.1 port=5432 dbname=notesdb user=postgres password=postgres".to_string());

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        // Spawn the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        Ok(DatabasePool {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn execute_query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>> {
        let client = self.client.lock().await;
        let rows = client.query(query, params).await?;
        Ok(rows)
    }

    pub async fn execute_query_one(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Option<Row>> {
        let client = self.client.lock().await;
        let rows = client.query(query, params).await?;
        Ok(rows.into_iter().next())
    }

    pub async fn execute_command(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64> {
        let client = self.client.lock().await;
        let affected = client.execute(query, params).await?;
        Ok(affected)
    }
}