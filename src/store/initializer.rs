use crate::store::DatabaseResult;
use crate::store::error::StoreError;
use crate::store::pool::Pool;

pub struct Initializer {}

impl Initializer {
    pub async fn create(url: &'_ str) -> DatabaseResult<Pool> {
        Pool::new(url).await
    }

    pub async fn migrate(pool: &Pool) -> DatabaseResult<()> {
        sqlx::migrate!()
            .run(pool.as_inner_ref())
            .await
            .map_err(|er| StoreError::MigrationFailed(er.to_string()))?;
        Ok(())
    }
}
