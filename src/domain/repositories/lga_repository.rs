use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::lga::{Lga, CreateLgaRequest, UpdateLgaRequest};
use crate::domain::value_objects::LgaCode;
use crate::errors::AppResult;

/// LGA repository interface
#[async_trait]
pub trait LgaRepository: Send + Sync {
    /// Create a new LGA
    async fn create(&self, request: &CreateLgaRequest, state_id: Uuid) -> AppResult<Lga>;
    
    /// Find LGA by ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Lga>>;
    
    /// Find LGA by name
    async fn find_by_name(&self, name: &str) -> AppResult<Option<Lga>>;
    
    /// Find LGA by code
    async fn find_by_code(&self, code: &LgaCode) -> AppResult<Option<Lga>>;
    
    /// Find LGAs by state ID
    async fn find_by_state_id(&self, state_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<Lga>>;
    
    /// Update LGA
    async fn update(&self, id: Uuid, request: &UpdateLgaRequest) -> AppResult<Lga>;
    
    /// Delete LGA
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Count LGAs by state
    async fn count_by_state(&self, state_id: Uuid) -> AppResult<u64>;
    
    /// Search LGAs by name or code
    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<Lga>>;
}
