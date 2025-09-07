use std::fmt;

use crate::repo::entity::error::BuilderError;
use crate::store::error::StoreError;

#[derive(Debug)]
pub enum ServiceError {
    Database(StoreError),
    Mapping(BuilderError),
    Auth(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::Database(e) => write!(f, "Store error: {e}"),
            ServiceError::Mapping(e) => write!(f, "Builder error: {e}"),
            ServiceError::Auth(e) => write!(f, "Auth error: {e}"),
        }
    }
}

impl From<StoreError> for ServiceError {
    fn from(er: StoreError) -> Self {
        ServiceError::Database(er)
    }
}

impl From<BuilderError> for ServiceError {
    fn from(er: BuilderError) -> Self {
        ServiceError::Mapping(er)
    }
}

impl std::error::Error for ServiceError {}
