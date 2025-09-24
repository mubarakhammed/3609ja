use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::lga::Lga;

/// LGA DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct LgaDto {
    pub id: Uuid,
    pub state_id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
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
