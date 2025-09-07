use async_trait::async_trait;

use crate::error::ServiceError;
use crate::repo::repo_context::RepoContext;

#[async_trait]
pub trait Handler {
    type TModel;
    type TResponse;

    async fn process(
        ctx: RepoContext,
        entity: Self::TModel,
    ) -> Result<Self::TResponse, ServiceError>;
}
