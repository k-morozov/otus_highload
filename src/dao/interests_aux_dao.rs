use async_trait::async_trait;

use crate::store::DatabaseResult;

use super::base_dao::BaseDao;


#[async_trait]
pub trait InterestsAuxDao : BaseDao {
    async fn get_id_by_name(&mut self, interest_name: &str,) -> DatabaseResult<uuid::Uuid> ;
}