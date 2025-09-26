use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::domain::entities::state::State;

/// State DTO for API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StateDto {
    /// Unique identifier for the state
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub id: Uuid,
    /// Name of the state
    #[schema(example = "Lagos")]
    pub name: String,
    /// State code (e.g., NG-LA)
    #[schema(example = "NG-LA")]
    pub code: String,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<State> for StateDto {
    fn from(state: State) -> Self {
        Self {
            id: state.id,
            name: state.name,
            code: state.code.to_string(),
            created_at: state.created_at,
            updated_at: state.updated_at,
        }
    }
}
