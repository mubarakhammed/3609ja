use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// LGA code value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LgaCode(String);

#[derive(Error, Debug)]
pub enum LgaCodeError {
    #[error("LGA code must be between 2 and 20 characters")]
    InvalidLength,
    #[error("LGA code must follow format: LGA-XXXX")]
    InvalidFormat,
}

impl LgaCode {
    /// Create a new LGA code
    pub fn new(code: String) -> Result<Self, LgaCodeError> {
        if code.len() < 2 || code.len() > 20 {
            return Err(LgaCodeError::InvalidLength);
        }

        if !code.starts_with("LGA-") {
            return Err(LgaCodeError::InvalidFormat);
        }

        Ok(Self(code))
    }

    /// Get the code as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LgaCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<LgaCode> for String {
    fn from(code: LgaCode) -> Self {
        code.0
    }
}
