use std::sync::Arc;

use crate::repo::cities::Cities;
use crate::repo::interests::Interests;
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

    // pub fn city_repo(&self) -> &Cities {
    //     self.0.city_repo.as_ref()
    // }

    pub fn interest_repo(&self) -> &Interests {
        self.0.interest_repo.as_ref()
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
    // city_repo: Box<Cities>,
    interest_repo: Box<Interests>,
    user_interest_repo: Box<UserInterests>,
    user_credentials_repo: Box<UserCredentials>,
}

unsafe impl Sync for RepoContextImpl {}
unsafe impl Send for RepoContextImpl {}

impl RepoContextImpl {
    fn new(pool: Arc<Pool>) -> Self {
        let user_repo = Box::new(Users::new());
        // let city_repo = Box::new(Cities::new());
        let interest_repo = Box::new(Interests::new());
        let user_interest_repo = Box::new(UserInterests::new());
        let user_credentials_repo = Box::new(UserCredentials::new());

        Self {
            pool,
            user_repo,
            // city_repo,
            interest_repo,
            user_interest_repo,
            user_credentials_repo,
        }
    }
}
