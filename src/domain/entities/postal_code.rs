use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_objects::{PostalCode as PostalCodeValue, Coordinates};

/// Postal code domain entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostalCode {
    pub id: Uuid,
    pub ward_id: Uuid,
    pub postal_code: PostalCodeValue,
    pub coordinates: Option<Coordinates>,
    pub urban: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostalCode {
    /// Create a new postal code
    pub fn new(
        ward_id: Uuid,
        postal_code: PostalCodeValue,
        coordinates: Option<Coordinates>,
        urban: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            ward_id,
            postal_code,
            coordinates,
            urban,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update coordinates
    pub fn update_coordinates(&mut self, coordinates: Option<Coordinates>) {
        self.coordinates = coordinates;
        self.updated_at = Utc::now();
    }

    /// Update urban status
    pub fn update_urban_status(&mut self, urban: bool) {
        self.urban = urban;
        self.updated_at = Utc::now();
    }

    /// Check if this postal code is in an urban area
    pub fn is_urban(&self) -> bool {
        self.urban
    }

    /// Get coordinates if available
    pub fn get_coordinates(&self) -> Option<&Coordinates> {
        self.coordinates.as_ref()
    }
}

/// Postal code creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostalCodeRequest {
    #[validate(length(min = 5, max = 10))]
    pub postal_code: String,
    
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub urban: bool,
}

/// Postal code update request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostalCodeRequest {
    #[validate(length(min = 5, max = 10))]
    pub postal_code: Option<String>,
    
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub urban: Option<bool>,
}
