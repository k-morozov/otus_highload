use async_trait::async_trait;

use crate::store::DatabaseResult;

#[async_trait]
pub trait Repository<T>: Sync + Send {
    async fn create<'a, E: sqlx::PgExecutor<'a>>(&self, e: E, entity: &T) -> DatabaseResult<()>;
    // async fn get_id(&self, entity: &T) -> DatabaseResult<Option<Uuid>>;
}

#[async_trait]
pub trait Repo: Send + Sync {
    async fn create<'a, E: sqlx::PgExecutor<'a>, T>(&self, e: E, entity: &T) -> DatabaseResult<()>;
}