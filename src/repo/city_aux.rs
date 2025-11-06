use async_trait::async_trait;

use super::repository::Repository;
use crate::store::DatabaseResult;

#[async_trait]
pub trait CityAux<I, T>: Repository<I, T> {
    async fn get_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &T,
    ) -> DatabaseResult<Option<I>>;
}
