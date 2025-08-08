use async_trait::async_trait;

use crate::error::ServiceError;
use crate::repo::repository::Repository;

#[async_trait]
pub trait Handler {
    type TEntity;

    async fn process(
        entity: &Self::TEntity,
        repo: &dyn Repository<Self::TEntity>,
    ) -> Result<(), ServiceError>;
}
