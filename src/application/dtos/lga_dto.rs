use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::domain::entities::lga::Lga;

/// LGA DTO for API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LgaDto {
    /// Unique identifier for the LGA
    #[schema(example = "650e8400-e29b-41d4-a716-446655440001")]
    pub id: Uuid,
    /// ID of the parent state
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub state_id: Uuid,
    /// Name of the LGA
    #[schema(example = "Ikeja")]
    pub name: String,
    /// LGA code (e.g., NG-LA-01)
    #[schema(example = "NG-LA-01")]
    pub code: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Lga> for LgaDto {
    fn from(lga: Lga) -> Self {
        Self {
            id: lga.id,
            state_id: lga.state_id,
            name: lga.name,
            code: lga.code.to_string(),
            created_at: lga.created_at,
            updated_at: lga.updated_at,
        }
    }
}
