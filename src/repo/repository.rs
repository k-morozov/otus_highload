use async_trait::async_trait;

use crate::store::DatabaseResult;

#[async_trait]
pub trait Repository<T>: Sync + Send {
    type TDatabase;

    async fn create<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &T,
    ) -> DatabaseResult<()>;
}
