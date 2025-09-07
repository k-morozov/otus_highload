use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    ConnectionFailed(String),
    MigrationFailed(String),
    ExecutionFailed(String),
    TransactionFailed(String),
    NoData(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
            StoreError::MigrationFailed(msg) => write!(f, "Migration failed: {msg}"),
            StoreError::ExecutionFailed(msg) => write!(f, "Execution failed: {msg}"),
            StoreError::TransactionFailed(msg) => write!(f, "Execution failed: {msg}"),
            StoreError::NoData(msg) => write!(f, "Execution failed: {msg}"),
        }
    }
}
