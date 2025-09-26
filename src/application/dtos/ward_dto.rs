use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::domain::entities::ward::Ward;

/// Ward DTO for API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WardDto {
    /// Unique identifier for the ward
    #[schema(example = "750e8400-e29b-41d4-a716-446655440001")]
    pub id: Uuid,
    /// ID of the parent LGA
    #[schema(example = "650e8400-e29b-41d4-a716-446655440001")]
    pub lga_id: Uuid,
    /// Name of the ward
    #[schema(example = "Ikeja")]
    pub name: String,
    /// Ward code (e.g., NG-LA-01-01)
    #[schema(example = "NG-LA-01-01")]
    pub code: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Ward> for WardDto {
    fn from(ward: Ward) -> Self {
        Self {
            id: ward.id,
            lga_id: ward.lga_id,
            name: ward.name,
            code: ward.code.to_string(),
            created_at: ward.created_at,
            updated_at: ward.updated_at,
        }
    }
}
