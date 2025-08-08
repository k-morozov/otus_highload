use async_trait::async_trait;
use sqlx::Execute;

use crate::repo::entity::interest;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

pub struct Interests {}

impl Interests {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository<interest::Entity> for Interests {
    async fn create<'a, E: sqlx::PgExecutor<'a>>(
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
            VALUES {};
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
