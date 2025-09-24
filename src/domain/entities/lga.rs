use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_objects::LgaCode;

/// LGA (Local Government Area) domain entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lga {
    pub id: Uuid,
    pub state_id: Uuid,
    pub name: String,
    pub code: LgaCode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Lga {
    /// Create a new LGA
    pub fn new(state_id: Uuid, name: String, code: LgaCode) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            state_id,
            name,
            code,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update LGA name
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// Update LGA code
    pub fn update_code(&mut self, code: LgaCode) {
        self.code = code;
        self.updated_at = Utc::now();
    }
}

/// LGA creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateLgaRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(length(min = 2, max = 20))]
    pub code: String,
}

/// LGA update request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLgaRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 2, max = 20))]
    pub code: Option<String>,
}
