use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::model::UserRegisterRequestBody;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;
use crate::store::pg_connection::PgConnection;
use crate::store::repository::Repository;

pub struct UserRepo {
    conn: Arc<PgConnection>,
}

impl UserRepo {
    pub fn new(conn: Arc<PgConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Repository<UserRegisterRequestBody> for UserRepo {
    async fn create(&self, entity: &UserRegisterRequestBody) -> DatabaseResult<()> {
        let query = "
            INSERT INTO users 
            (
                id,
                name,
                email
            ) 
            VALUES (1, $1, 'test@email.com');
            ";

        info!("user repo call create with query={}", query);

        let mut conn = self.conn.connection.lock().await;

        match conn.as_mut() {
            Some(c) => {
                sqlx::query(query)
                    .bind(&entity.name)
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
