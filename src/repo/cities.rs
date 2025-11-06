use async_trait::async_trait;
use sqlx::{Execute, Row};
use uuid::Uuid;

use crate::repo::entity::city;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

use super::city_aux::CityAux;

pub struct Cities {}

impl Cities {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository<Uuid, city::Entity> for Cities {
    type TDatabase = sqlx::Postgres;

    async fn create<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &city::Entity,
    ) -> DatabaseResult<()> {
        let raw_query = "
            INSERT INTO cities 
            (
                city_id,
                city_name
            ) 
            VALUES ($1, $2);
        ";

        let city_id = Uuid::new_v4();
        let query = sqlx::query(raw_query).bind(city_id).bind(&entity.city_name);

        tracing::info!("city repo call create with query={}", query.sql());

        query
            .execute(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }

    async fn get_by_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        _e: E,
        _id: &Uuid,
    ) -> DatabaseResult<Option<city::Entity>> {
        todo!()
    }
}

#[async_trait]
impl CityAux<Uuid, city::Entity> for Cities {
    async fn get_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &city::Entity,
    ) -> DatabaseResult<Option<Uuid>> {
        let raw_query = "
            SELECT city_id FROM cities 
            WHERE city_name=$1;
            ";

        let query = sqlx::query(raw_query).bind(&entity.city_name);

        tracing::info!("city repo call get_id with query={}", query.sql());

        let row = match query.fetch_one(e).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => {
                return Ok(None);
            }
            Err(e) => {
                return Err(StoreError::ExecutionFailed(e.to_string()));
            }
        };

        if row.is_empty() {
            return Ok(None);
        }

        let res: Uuid = row.get(0);

        Ok(Some(res))
    }
}
