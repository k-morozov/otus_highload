use std::sync::Arc;

use async_trait::async_trait;
use sqlx::Execute;
use uuid::Uuid;

use crate::model::UserRegisterRequestBody;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;
use crate::store::pg_connection::PgConnection;
use crate::repo::repository::Repository;

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
        match self.conn.lock().await.as_mut() {
            Some(c) => {
                let raw_query = "
                    INSERT INTO users 
                    (
                        id,
                        name,
                        surname,
                        birth_date
                    ) 
                    VALUES ($1, $2, $3, $4::date);
                    ";

                let user_id = Uuid::new_v4();
                let query = sqlx::query(raw_query)
                    .bind(user_id)
                    .bind(&entity.name)
                    .bind(&entity.surname)
                    .bind(&entity.birth_date);

                tracing::info!("user repo call create with query={}", query.sql());

                query
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
