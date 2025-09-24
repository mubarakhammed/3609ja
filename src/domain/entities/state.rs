use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_objects::StateCode;

/// State domain entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    pub code: StateCode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl State {
    /// Create a new state
    pub fn new(name: String, code: StateCode) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            code,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update state name
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// Update state code
    pub fn update_code(&mut self, code: StateCode) {
        self.code = code;
        self.updated_at = Utc::now();
    }
}

/// State creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateStateRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(length(min = 2, max = 10))]
    pub code: String,
}

/// State update request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStateRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 2, max = 10))]
    pub code: Option<String>,
}
