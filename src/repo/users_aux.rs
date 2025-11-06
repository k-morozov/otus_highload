use async_trait::async_trait;

use super::repository::Repository;
use crate::store::DatabaseResult;

#[async_trait]
pub trait UsersAux<I, T>: Repository<I, T> {
    async fn get_by_login<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        login: &str,
    ) -> DatabaseResult<I>;
}
