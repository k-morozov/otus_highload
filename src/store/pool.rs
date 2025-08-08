use crate::store::DatabaseResult;
use crate::store::error::StoreError; // seems worth

type InternalPool = sqlx::PgPool;

pub struct Pool {
    inner: InternalPool,
}

impl Pool {
    pub async fn new(url: &str) -> DatabaseResult<Self> {
        Ok(Self {
            inner: InternalPool::connect(url)
                .await
                .map_err(|e| StoreError::ConnectionFailed(e.to_string()))?,
        })
    }

    pub fn as_inner_ref(&self) -> &InternalPool {
        &self.inner
    }
}
