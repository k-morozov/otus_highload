use async_trait::async_trait;
use sqlx::{Execute, Row};

use crate::repo::entity::interest;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

use super::interests_aux::InterestsAux;

pub struct Interests {}

impl Interests {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository<interest::Entity> for Interests {
    type TDatabase = sqlx::Postgres;

    async fn create<'a, E: sqlx::Executor<'a, Database=Self::TDatabase>>(
        &self,
        e: E,
        entity: &interest::Entity,
    ) -> DatabaseResult<()> {
        let inserted: Vec<String> = (0..entity.interests.len())
            .map(|num| format!("(${}, ${})", 2 * num + 1, 2 * num + 2))
            .collect();

        let raw_query = format!(
            "
            INSERT INTO interests 
            (
                interest_id,
                interest_name
            ) 
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            inserted.join(", ")
        );

        let mut query = sqlx::query(raw_query.as_str());

        for (interest_id, interest_name) in &entity.interests {
            query = query.bind(interest_id).bind(interest_name);
        }

        tracing::info!("interests repo call create with query={}", query.sql());

        query
            .execute(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }
}

#[async_trait]
impl InterestsAux<interest::Entity> for Interests {
    async fn get_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        interest_name: &str,
    ) -> DatabaseResult<uuid::Uuid> {
        let raw_query = "
            SELECT interest_id FROM interests
            WHERE interest_name=$1
        ";

        let query = sqlx::query(raw_query).bind(interest_name);

        tracing::info!("interests repo call get_id with query={}", query.sql());

        let row = query
            .fetch_one(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        let interest_id: uuid::Uuid = row.get(0);

        return Ok(interest_id);
    }
}