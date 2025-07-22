use std::fmt;

use crate::store::error::StoreError;

#[derive(Debug)]
pub enum ServiceError {
    Database(StoreError),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::Database(e) => write!(f, "Store error: {}", e),
        }
    }
}

impl From<StoreError> for ServiceError {
    fn from(er: StoreError) -> Self {
        ServiceError::Database(er)
    }
}

impl std::error::Error for ServiceError {}
