use std::fmt;

#[derive(Debug)]
pub enum BuilderError {
    NotEnoughElement(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuilderError::NotEnoughElement(msg) => {
                write!(f, "Not all elements were initialized: {msg}")
            }
        }
    }
}
