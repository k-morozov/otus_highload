use async_trait::async_trait;
use uuid::Uuid;

use super::repository::Repository;
use crate::store::DatabaseResult;

#[async_trait]
pub trait CityAux<T>: Repository<T> {
    async fn get_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &T,
    ) -> DatabaseResult<Option<Uuid>>;
}
