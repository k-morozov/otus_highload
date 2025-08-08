use std::sync::Arc;

use crate::error::ServiceError;
use crate::repo::repo_context::RepoContext;
use crate::store::initializer::Initializer;

const URL: &str = "postgres://dev:123@localhost/app_db";

pub struct AppState {
    pub ctx: RepoContext,
}

impl AppState {
    pub async fn new() -> Result<AppState, ServiceError> {
        // @todo reduce Arc
        let pool = Arc::new(Initializer::create(URL).await?);

        Initializer::migrate(pool.as_ref()).await?;

        let ctx = RepoContext::new(pool);

        Ok(AppState { ctx })
    }
}
