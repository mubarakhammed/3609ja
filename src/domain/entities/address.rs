use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::{State, Lga, Ward, PostalCode};
// use crate::domain::value_objects::{StateCode, LgaCode, WardCode, PostalCode as PostalCodeValue};

/// Complete address aggregate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub state: State,
    pub lga: Lga,
    pub ward: Ward,
    pub postal_code: PostalCode,
}

impl Address {
    /// Create a new address
    pub fn new(state: State, lga: Lga, ward: Ward, postal_code: PostalCode) -> Self {
        Self {
            state,
            lga,
            ward,
            postal_code,
        }
    }

    /// Get the full address as a formatted string
    pub fn to_string(&self) -> String {
        format!(
            "{} {}, {} {}",
            self.postal_code.postal_code,
            self.ward.name,
            self.lga.name,
            self.state.name
        )
    }

    /// Check if this is a valid Nigerian address
    pub fn is_valid(&self) -> bool {
        // Business rules for address validation
        self.state.code.as_str().starts_with("NG-") &&
        self.lga.code.as_str().starts_with("NG-") &&
        self.ward.code.as_str().starts_with("NG-") &&
        self.postal_code.postal_code.as_str().len() >= 5
    }
}

/// Address validation request
#[derive(Debug, Deserialize, Validate)]
pub struct AddressValidationRequest {
    #[validate(length(min = 1))]
    pub state: String,
    
    #[validate(length(min = 1))]
    pub lga: String,
    
    #[validate(length(min = 1))]
    pub ward: String,
    
    #[validate(length(min = 1))]
    pub postal_code: String,
}

/// Address validation response
#[derive(Debug, Serialize)]
pub struct AddressValidationResponse {
    pub valid: bool,
    pub canonical: Option<Address>,
    pub suggestions: Vec<AddressSuggestion>,
}

/// Address suggestion
#[derive(Debug, Serialize)]
pub struct AddressSuggestion {
    pub state: Option<State>,
    pub lga: Option<Lga>,
    pub ward: Option<Ward>,
    pub postal_code: Option<PostalCode>,
    pub reason: String,
    pub confidence: f64, // 0.0 to 1.0
}
