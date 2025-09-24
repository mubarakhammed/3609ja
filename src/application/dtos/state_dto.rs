use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::state::State;

/// State DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct StateDto {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
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
