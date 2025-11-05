use std::sync::Arc;

use crate::repo::user_credentials::UserCredentials;
use crate::repo::user_interests::UserInterests;
use crate::repo::users::Users;
use crate::store::pool::Pool;

#[derive(Clone)]
pub struct RepoContext(Arc<RepoContextImpl>);

impl RepoContext {
    pub fn new(pool: Arc<Pool>) -> Self {
        RepoContext(Arc::new(RepoContextImpl::new(pool)))
    }

    pub fn get_pool(&self) -> Arc<Pool> {
        Arc::clone(&self.0.pool)
    }

    pub fn user_repo(&self) -> &Users {
        self.0.user_repo.as_ref()
    }


    pub fn user_interest_repo(&self) -> &UserInterests {
        self.0.user_interest_repo.as_ref()
    }

    pub fn user_credentials_repo(&self) -> &UserCredentials {
        self.0.user_credentials_repo.as_ref()
    }
}

struct RepoContextImpl {
    pool: Arc<Pool>,
    user_repo: Box<Users>,
    user_interest_repo: Box<UserInterests>,
    user_credentials_repo: Box<UserCredentials>,
}

unsafe impl Sync for RepoContextImpl {}
unsafe impl Send for RepoContextImpl {}

impl RepoContextImpl {
    fn new(pool: Arc<Pool>) -> Self {
        let user_repo = Box::new(Users::new());
        let user_interest_repo = Box::new(UserInterests::new());
        let user_credentials_repo = Box::new(UserCredentials::new());

        Self {
            pool,
            user_repo,
            user_interest_repo,
            user_credentials_repo,
        }
    }
}
