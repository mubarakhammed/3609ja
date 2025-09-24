use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ward::{Ward, CreateWardRequest, UpdateWardRequest};
use crate::domain::value_objects::WardCode;
use crate::errors::AppResult;

/// Ward repository interface
#[async_trait]
pub trait WardRepository: Send + Sync {
    /// Create a new ward
    async fn create(&self, request: &CreateWardRequest, lga_id: Uuid) -> AppResult<Ward>;
    
    /// Find ward by ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Ward>>;
    
    /// Find ward by name
    async fn find_by_name(&self, name: &str) -> AppResult<Option<Ward>>;
    
    /// Find ward by code
    async fn find_by_code(&self, code: &WardCode) -> AppResult<Option<Ward>>;
    
    /// Find wards by LGA ID
    async fn find_by_lga_id(&self, lga_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<Ward>>;
    
    /// Update ward
    async fn update(&self, id: Uuid, request: &UpdateWardRequest) -> AppResult<Ward>;
    
    /// Delete ward
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Count wards by LGA
    async fn count_by_lga(&self, lga_id: Uuid) -> AppResult<u64>;
    
    /// Search wards by name or code
    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<Ward>>;
}
