use async_trait::async_trait;
use tracing::{Level, event};

use crate::error::ServiceError;
use crate::handlers::handler::Handler;
use crate::model::UserRegisterRequestBody;
use crate::store::repository::Repository;

pub struct UserRegister;

#[async_trait]
impl Handler for UserRegister {
    type TEntity = UserRegisterRequestBody;

    async fn process(
        entity: &Self::TEntity,
        repo: &dyn Repository<Self::TEntity>,
    ) -> Result<(), ServiceError> {
        event!(Level::INFO, "process_user_register: entity={:?}", entity);
        let _res = repo
            .create(&entity)
            .await
            .map_err(|e| ServiceError::Database(e))?;

        Ok(())
    }
}
