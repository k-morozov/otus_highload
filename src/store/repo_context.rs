use std::sync::Arc;

use crate::store::pg_connection::PgConnection;
use crate::store::user_repo::UserRepo;

pub struct RepoContext {
    user_repo: Box<UserRepo>,
}

unsafe impl Sync for RepoContext {}
unsafe impl Send for RepoContext {}

impl RepoContext {
    pub fn new(conn: Arc<PgConnection>) -> Self {
        let user_repo = Box::new(UserRepo::new(conn));
        Self { user_repo }
    }

    pub fn user_repo(&self) -> &UserRepo {
        self.user_repo.as_ref()
    }
}
