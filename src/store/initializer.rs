use crate::store::error::StoreError;
use crate::store::pool::Pool;
use crate::store::{DatabaseResult, schema};

pub struct Initializer {}

impl Initializer {
    pub async fn create(url: &'_ str) -> DatabaseResult<Pool> {
        Pool::new(url).await
    }

    pub async fn migrate(pool: &Pool) -> DatabaseResult<()> {
        for query in schema::CREATE_TABLES {
            sqlx::query(query)
                .execute(pool.as_inner_ref())
                .await
                .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;
        }
        Ok(())
    }
}
