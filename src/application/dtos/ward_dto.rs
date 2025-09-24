use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::ward::Ward;

/// Ward DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct WardDto {
    pub id: Uuid,
    pub lga_id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
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
