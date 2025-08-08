use crate::store::DatabaseResult;
use crate::store::error::StoreError;
use crate::store::pg_connection::PgConnection;

pub struct Initializer {}

impl Initializer {
    pub fn create() -> DatabaseResult<PgConnection> {
        Ok(PgConnection::new())
    }

    pub async fn migrate(conn: &PgConnection) -> DatabaseResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id UUID UNIQUE PRIMARY KEY,
                name VARCHAR NOT NULL,
                surname VARCHAR NOT NULL,
                birth_date DATE NOT NULL
            )
            "#;

        match conn.lock().await.as_mut() {
            Some(c) => {
                sqlx::query(query)
                    .execute(c)
                    .await
                    .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;
                return Ok(());
            }
            None => {
                return Err(StoreError::ConnectionFailed(
                    "PgDatabase has empty connection".to_string(),
                ));
            }
        }
    }
}
