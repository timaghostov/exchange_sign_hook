#[derive(Debug)]
pub enum SignError {
    Other(String),
}

impl SignError {
    pub fn other(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

pub type SignResult = Result<String, SignError>;
