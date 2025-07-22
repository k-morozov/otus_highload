use crate::store::error::StoreError;

pub mod error;
pub mod initializer;
pub mod pg_connection;
pub mod repo_context;
pub mod repository;
pub mod user_repo;

type DatabaseResult<T> = Result<T, StoreError>;
