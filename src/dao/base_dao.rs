use async_trait::async_trait;

use crate::store::DatabaseResult;

#[async_trait]
pub trait BaseDao {
    type TEntity;

    async fn create(&mut self, entry: &Self::TEntity) -> DatabaseResult<()>;

    fn find_by_id() -> Self::TEntity;

    fn find_by_ids() -> Self::TEntity;

    fn find_all();
    
    fn delete_all();
}