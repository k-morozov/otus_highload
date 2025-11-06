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
impl Repository<uuid::Uuid, user_interests::Entity> for UserInterests {
    type TDatabase = sqlx::Postgres;

    async fn create<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &user_interests::Entity,
    ) -> DatabaseResult<()> {
        let inserts: Vec<String> = (0..entity.interests.len())
            .map(|num| format!("(${}, ${})", 2 * num + 1, 2 * num + 2))
            .collect();

        let raw_query = format!(
            "
            INSERT INTO user_interests 
            (
                user_id,
                interest_id
            ) 
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            inserts.join(", ")
        );
        let mut query = sqlx::query(raw_query.as_str());

        for interest_id in &entity.interests {
            query = query.bind(entity.user_id).bind(interest_id);

            tracing::info!("user_interests repo call create with query={}", query.sql());
        }

        query
            .execute(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }

    async fn get_by_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        _e: E,
        _id: &uuid::Uuid,
    ) -> DatabaseResult<Option<user_interests::Entity>> {
        unreachable!()
    }
}
