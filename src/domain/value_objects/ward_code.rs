use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Ward code value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WardCode(String);

#[derive(Error, Debug)]
pub enum WardCodeError {
    #[error("Ward code must be between 2 and 25 characters")]
    InvalidLength,
    #[error("Ward code must follow format: NG-XX-YY-ZZ")]
    InvalidFormat,
}

impl WardCode {
    /// Create a new ward code
    pub fn new(code: String) -> Result<Self, WardCodeError> {
        if code.len() < 2 || code.len() > 25 {
            return Err(WardCodeError::InvalidLength);
        }
        
        if !code.starts_with("NG-") {
            return Err(WardCodeError::InvalidFormat);
        }
        
        Ok(Self(code))
    }

    /// Get the code as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WardCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<WardCode> for String {
    fn from(code: WardCode) -> Self {
        code.0
    }
}
