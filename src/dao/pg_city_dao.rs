use async_trait::async_trait;
use sqlx::{Execute};
use uuid::Uuid;
use sqlx::Row;

use crate::store::DatabaseResult;
use crate::store::error::StoreError;

use super::base_dao::BaseDao;
use super::city_aux_dao::CityAuxDao;
use super::city_entity::CityEntity;


pub struct PgCityDao<'a> {
    e: &'a mut sqlx::PgConnection
}

impl<'a> PgCityDao<'a> {
    pub fn new(e: &'a mut sqlx::PgConnection) -> Self {
        Self {e}
    }
}

unsafe impl<'a> Send for PgCityDao<'a> {}
unsafe impl<'a> Sync for PgCityDao<'a> {}

#[async_trait]
impl<'a> BaseDao for PgCityDao<'a> {
    type TEntity = CityEntity;

    async fn create(&mut self, entity: &Self::TEntity) -> DatabaseResult<()> {
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
impl<'a> CityAuxDao for PgCityDao<'a> {   
    async fn get_id(&mut self, entry: &Self::TEntity) -> DatabaseResult<Option<Uuid>> {
        let raw_query = "
            SELECT city_id FROM cities 
            WHERE city_name=$1;
            ";

        let query = sqlx::query(raw_query).bind(&entry.city_name);

        tracing::info!("city repo call get_id with query={}", query.sql());

        let row = match query.fetch_one(&mut *self.e).await {
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