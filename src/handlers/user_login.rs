use async_trait::async_trait;
use tracing::Level;

use crate::auth::{SECRET_JWT_TOKEN, generate_jwt_token};
use crate::error::ServiceError;
use crate::handlers::handler::Handler;
use crate::model::{UserLoginRequestBody, UserLoginResponseBody};
use crate::repo::repo_context::RepoContext;
use crate::repo::users_aux::UsersAux;
use crate::utils;

pub struct UserLogin;

#[async_trait]
impl Handler for UserLogin {
    type TModel = UserLoginRequestBody;
    type TResponse = UserLoginResponseBody;

    async fn process(
        ctx: RepoContext,
        model: Self::TModel,
    ) -> Result<Self::TResponse, ServiceError> {
        tracing::event!(Level::INFO, "UserLogin::process model={:?}", model);

        let user_id = ctx
            .user_repo()
            .get_by_login(ctx.get_pool().as_inner_ref(), model.login.as_str())
            .await?;

        if !ctx
            .user_credentials_repo()
            .check_password(
                ctx.get_pool().as_inner_ref(),
                user_id,
                &utils::crypto_password(model.password.as_str()),
            )
            .await?
        {
            return Err(ServiceError::Auth("Incorrect password".to_string()));
        }

        let token = generate_jwt_token(&user_id, SECRET_JWT_TOKEN)
            .map_err(|_| ServiceError::Auth("failed generate token".to_string()))?;

        let response = Self::TResponse { token };

        Ok(response)
    }
}
