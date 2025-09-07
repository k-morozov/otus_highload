use async_trait::async_trait;
use sqlx::{Execute, Row};

use crate::repo::entity::user_credentials;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

pub struct UserCredentials {}

impl UserCredentials {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn check_password<'a, E: sqlx::PgExecutor<'a>>(
        &self,
        e: E,
        user_id: uuid::Uuid,
        password: &str,
    ) -> DatabaseResult<bool> {
        let raw_query = "
            SELECT password 
            FROM user_credentials
            WHERE
            user_id=$1;
        ";

        let query = sqlx::query(raw_query).bind(user_id);

        tracing::debug!(
            "user_credentials repo call check_password with query={}",
            query.sql()
        );

        let result_query = query
            .fetch_optional(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        let row = result_query
            .ok_or_else(|| StoreError::NoData(String::from("no user with the user_id")))?;

        tracing::debug!("got the data for auth {row:?}");

        if row.is_empty() {
            // check empty answer
            return Err(StoreError::NoData(String::from("no the user_id")));
        }

        let user_password: String = row.get(0);

        if password == user_password {
            return Ok(true);
        }

        Ok(false)
    }
}

#[async_trait]
impl Repository<user_credentials::Entity> for UserCredentials {
    async fn create<'a, E: sqlx::PgExecutor<'a>>(
        &self,
        e: E,
        entity: &user_credentials::Entity,
    ) -> DatabaseResult<()> {
        let raw_query = "
            INSERT INTO user_credentials
            (
                user_id,
                password
            ) 
            VALUES ($1, $2);
        ";

        let query = sqlx::query(raw_query)
            .bind(entity.user_id)
            .bind(&entity.password);

        tracing::info!(
            "user_credentials repo call create with query={}",
            query.sql()
        );

        query
            .execute(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }
}
