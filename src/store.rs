use crate::store::error::StoreError;

pub mod error;
pub mod initializer;
pub mod pool;

pub type DatabaseResult<T> = Result<T, StoreError>;
