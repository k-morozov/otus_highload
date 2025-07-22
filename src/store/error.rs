use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    ConnectionFailed(String),
    ExecutionFailed(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            StoreError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
        }
    }
}
