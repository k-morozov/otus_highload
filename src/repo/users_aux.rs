use async_trait::async_trait;
use uuid::Uuid;

use super::repository::Repository;
use crate::store::DatabaseResult;

#[async_trait]
pub trait UsersAux<T>: Repository<T> {
    async fn get_by_login<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        login: &str,
    ) -> DatabaseResult<Uuid>;

    async fn get_by_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        user_id: &str,
    ) -> DatabaseResult<(String, String)>;
}
