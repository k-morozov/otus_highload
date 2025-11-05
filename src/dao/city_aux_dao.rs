use async_trait::async_trait;
use uuid::Uuid;

use crate::store::DatabaseResult;

use super::base_dao::BaseDao;


#[async_trait]
pub trait CityAuxDao : BaseDao {
    async fn get_id(&mut self, entry: &<Self as BaseDao>::TEntity) -> DatabaseResult<Option<Uuid>> ;
}