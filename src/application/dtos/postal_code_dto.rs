use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::domain::entities::postal_code::PostalCode;

/// Postal code DTO for API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PostalCodeDto {
    /// Unique identifier for the postal code
    #[schema(example = "850e8400-e29b-41d4-a716-446655440001")]
    pub id: Uuid,
    /// ID of the parent ward
    #[schema(example = "750e8400-e29b-41d4-a716-446655440001")]
    pub ward_id: Uuid,
    /// Postal code
    #[schema(example = "100001")]
    pub postal_code: String,
    /// Latitude coordinate
    #[schema(example = 6.6059)]
    pub lat: Option<f64>,
    /// Longitude coordinate
    #[schema(example = 3.3515)]
    pub lng: Option<f64>,
    /// Whether this is an urban area
    #[schema(example = true)]
    pub urban: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
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
