use async_trait::async_trait;

use crate::store::DatabaseResult;

#[async_trait]
pub trait Repository<I, T>: Sync + Send {
    type TDatabase;

    async fn create<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &T,
    ) -> DatabaseResult<()>;

    async fn get_by_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        id: &I,
    ) -> DatabaseResult<Option<T>>;
}
