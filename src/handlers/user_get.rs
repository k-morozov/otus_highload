use async_trait::async_trait;
use tracing::Level;

use crate::error::ServiceError;
use crate::handlers::handler::Handler;
use crate::model::{UserGetRequestBody, UserGetResponseBody};
use crate::repo::repo_context::RepoContext;
use crate::repo::users_aux::UsersAux;

pub struct UserGet;

#[async_trait]
impl Handler for UserGet {
    type TModel = UserGetRequestBody;
    type TResponse = UserGetResponseBody;

    async fn process(
        ctx: RepoContext,
        model: Self::TModel,
    ) -> Result<Self::TResponse, ServiceError> {
        tracing::event!(Level::INFO, "UserGet::process model={:?}", model);

        let res = ctx
            .user_repo()
            .get_by_id(ctx.get_pool().as_inner_ref(), &model.id)
            .await?;

        let response = Self::TResponse {
            name: res.0,
            surname: res.1,
        };

        Ok(response)
    }
}
