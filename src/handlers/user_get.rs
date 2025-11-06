use std::str::FromStr;

use async_trait::async_trait;
use tracing::Level;
use uuid::Uuid;

use crate::error::ServiceError;
use crate::handlers::handler::Handler;
use crate::model::{UserGetRequestBody, UserGetResponseBody};
use crate::repo::repo_context::RepoContext;
use crate::repo::repository::Repository;
use crate::store::error::StoreError::NoData;

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
            .get_by_id(
                ctx.get_pool().as_inner_ref(),
                &Uuid::from_str(&model.id)
                    .map_err(|_| ServiceError::Auth("Broken id".to_string()))?,
            )
            .await?;

        match res {
            Some(entity) => {
                return Ok(Self::TResponse {
                    name: entity.name,
                    surname: entity.surname,
                });
            }
            None => {
                return Err(ServiceError::Database(NoData("User not found".to_string())));
            }
        }
    }
}
