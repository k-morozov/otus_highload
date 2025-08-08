use async_trait::async_trait;
use sqlx::Execute;

use super::entity::user_interests;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

pub struct UserInterests {}

impl UserInterests {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository<user_interests::Entity> for UserInterests {
    async fn create<'a, E: sqlx::PgExecutor<'a>>(
        &self,
        e: E,
        entity: &user_interests::Entity,
    ) -> DatabaseResult<()> {
        for interest_id in &entity.interests {
            let raw_query = "
                INSERT INTO user_interests 
                (
                    user_id,
                    interest_id
                ) 
                VALUES ($1, $2);
            ";

            let query = sqlx::query(raw_query)
                .bind(entity.user_id)
                .bind(interest_id);

            tracing::info!("user_interests repo call create with query={}", query.sql());

            query
                .execute(e)
                .await
                .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

            break;
        }

        return Ok(());
    }
}
