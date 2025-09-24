use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_objects::WardCode;

/// Ward domain entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ward {
    pub id: Uuid,
    pub lga_id: Uuid,
    pub name: String,
    pub code: WardCode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ward {
    /// Create a new ward
    pub fn new(lga_id: Uuid, name: String, code: WardCode) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            lga_id,
            name,
            code,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update ward name
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// Update ward code
    pub fn update_code(&mut self, code: WardCode) {
        self.code = code;
        self.updated_at = Utc::now();
    }
}

/// Ward creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateWardRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(length(min = 2, max = 25))]
    pub code: String,
}

/// Ward update request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWardRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 2, max = 25))]
    pub code: Option<String>,
}
