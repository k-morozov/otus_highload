use async_trait::async_trait;

use crate::store::DatabaseResult;

use super::repository::Repository;

#[async_trait]
pub trait InterestsAux<I, T>: Repository<I, T> {
    async fn get_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        interest_name: &str,
    ) -> DatabaseResult<I>;
}
