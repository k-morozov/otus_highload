use crate::store::error::StoreError;

pub mod error;
pub mod initializer;
pub mod pool;

mod schema;

pub type DatabaseResult<T> = Result<T, StoreError>;
