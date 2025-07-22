use async_trait::async_trait;

use crate::store::DatabaseResult;

#[async_trait]
pub trait Repository<T>: Sync + Send {
    async fn create(&self, entity: &T) -> DatabaseResult<()>;
}
