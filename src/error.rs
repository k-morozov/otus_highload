use std::fmt;

use crate::repo::entity;
use crate::dao;
use crate::store::error::StoreError;

#[derive(Debug)]
pub enum ServiceError {
    Database(StoreError),
    Mapping(entity::error::BuilderError),
    Mapping2(dao::error::BuilderError),
    Auth(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::Database(e) => write!(f, "Store error: {e}"),
            ServiceError::Mapping(e) => write!(f, "Builder error: {e}"),
            ServiceError::Mapping2(e) => write!(f, "Builder error: {e}"),
            ServiceError::Auth(e) => write!(f, "Auth error: {e}"),
        }
    }
}

impl From<StoreError> for ServiceError {
    fn from(er: StoreError) -> Self {
        ServiceError::Database(er)
    }
}

impl From<entity::error::BuilderError> for ServiceError {
    fn from(er: entity::error::BuilderError) -> Self {
        ServiceError::Mapping(er)
    }
}

impl From<dao::error::BuilderError> for ServiceError {
    fn from(er: dao::error::BuilderError) -> Self {
        ServiceError::Mapping2(er)
    }
}

impl std::error::Error for ServiceError {}
