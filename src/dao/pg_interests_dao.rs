use async_trait::async_trait;
use sqlx::{Execute};
use uuid::Uuid;
use sqlx::Row;

use crate::store::DatabaseResult;
use crate::store::error::StoreError;

use super::base_dao::BaseDao;
use super::interest_entity::InterestEntity;
use super::interests_aux_dao::InterestsAuxDao;


pub struct PgInterestsDao<'a> {
    e: &'a mut sqlx::PgConnection
}

impl<'a> PgInterestsDao<'a> {
    pub fn new(e: &'a mut sqlx::PgConnection) -> Self {
        Self {e}
    }
}

unsafe impl<'a> Send for PgInterestsDao<'a> {}
unsafe impl<'a> Sync for PgInterestsDao<'a> {}

#[async_trait]
impl<'a> BaseDao for PgInterestsDao<'a> {
    type TEntity = InterestEntity;

    async fn create(&mut self, entity: &Self::TEntity) -> DatabaseResult<()> {
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
            .execute(&mut *self.e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }

    
    fn find_by_id() -> Self::TEntity {
        todo!()
    }

    fn find_by_ids() -> Self::TEntity {
        todo!()
    }

    fn find_all() {
        todo!()
    }

    fn delete_all() {
        todo!()
    }
}


#[async_trait]
impl<'a> InterestsAuxDao for PgInterestsDao<'a> {   
    async fn get_id_by_name(&mut self, interest_name: &str,) -> DatabaseResult<uuid::Uuid> {
        let raw_query = "
            SELECT interest_id FROM interests
            WHERE interest_name=$1
        ";

        let query = sqlx::query(raw_query).bind(interest_name);

        tracing::info!("interests repo call get_id with query={}", query.sql());

        let row = query
            .fetch_one(&mut *self.e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        let interest_id: uuid::Uuid = row.get(0);

        return Ok(interest_id);
    } 
}