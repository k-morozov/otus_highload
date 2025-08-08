use crate::store::error::StoreError;

pub mod error;
pub mod initializer;
pub mod pg_connection;

pub type DatabaseResult<T> = Result<T, StoreError>;
