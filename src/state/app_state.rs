use std::sync::Arc;

use crate::error::ServiceError;
use crate::store::initializer::Initializer;
use crate::repo::repo_context::RepoContext;

const URL: &str = "postgres://dev:123@localhost/app_db";

pub struct AppState {
    pub ctx: RepoContext,
}

impl AppState {
    pub async fn new() -> Result<AppState, ServiceError> {
        let conn = Arc::new(Initializer::create()?);
        conn.connect(URL).await?;

        Initializer::migrate(conn.as_ref()).await?;

        let ctx = RepoContext::new(conn);

        Ok(AppState { ctx })
    }
}
