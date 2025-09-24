use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::postal_code::PostalCode;

/// Postal code DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCodeDto {
    pub id: Uuid,
    pub ward_id: Uuid,
    pub postal_code: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub urban: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<PostalCode> for PostalCodeDto {
    fn from(postal_code: PostalCode) -> Self {
        Self {
            id: postal_code.id,
            ward_id: postal_code.ward_id,
            postal_code: postal_code.postal_code.to_string(),
            lat: postal_code.coordinates.as_ref().map(|c| c.latitude),
            lng: postal_code.coordinates.as_ref().map(|c| c.longitude),
            urban: postal_code.urban,
            created_at: postal_code.created_at,
            updated_at: postal_code.updated_at,
        }
    }
}
