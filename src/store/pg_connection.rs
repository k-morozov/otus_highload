use sqlx::Connection;
use tokio::sync::Mutex;

use crate::store::DatabaseResult;
use crate::store::error::StoreError; // seems worth

pub struct PgConnection {
    pub connection: tokio::sync::Mutex<Option<sqlx::PgConnection>>,
}

impl PgConnection {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }
}

impl PgConnection {
    pub async fn connect(&self, url: &'_ str) -> DatabaseResult<()> {
        let mut conn = self.connection.lock().await;
        let _ = conn.insert(
            sqlx::PgConnection::connect(url)
                .await
                .map_err(|e| StoreError::ConnectionFailed(e.to_string()))?,
        );

        Ok(())
    }

    async fn disconect(&self) -> DatabaseResult<()> {
        todo!()
    }
}
