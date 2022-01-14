use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("Sign Error: {0}")]
    Other(String),
}

impl SignError {
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

pub type SignResult = Result<String, SignError>;
