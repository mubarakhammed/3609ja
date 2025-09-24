use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Postal code value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostalCode(String);

#[derive(Error, Debug)]
pub enum PostalCodeError {
    #[error("Postal code must be between 5 and 10 characters")]
    InvalidLength,
    #[error("Postal code must contain only digits")]
    InvalidFormat,
}

impl PostalCode {
    /// Create a new postal code
    pub fn new(code: String) -> Result<Self, PostalCodeError> {
        if code.len() < 5 || code.len() > 10 {
            return Err(PostalCodeError::InvalidLength);
        }
        
        if !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(PostalCodeError::InvalidFormat);
        }
        
        Ok(Self(code))
    }

    /// Get the code as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PostalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PostalCode> for String {
    fn from(code: PostalCode) -> Self {
        code.0
    }
}
