use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// State code value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StateCode(String);

#[derive(Error, Debug)]
pub enum StateCodeError {
    #[error("State code must be between 2 and 10 characters")]
    InvalidLength,
    #[error("State code must follow format: NG-XX")]
    InvalidFormat,
}

impl StateCode {
    /// Create a new state code
    pub fn new(code: String) -> Result<Self, StateCodeError> {
        if code.len() < 2 || code.len() > 10 {
            return Err(StateCodeError::InvalidLength);
        }
        
        if !code.starts_with("NG-") {
            return Err(StateCodeError::InvalidFormat);
        }
        
        Ok(Self(code))
    }

    /// Get the code as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StateCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<StateCode> for String {
    fn from(code: StateCode) -> Self {
        code.0
    }
}
