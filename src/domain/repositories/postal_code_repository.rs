use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::postal_code::{PostalCode, CreatePostalCodeRequest, UpdatePostalCodeRequest};
use crate::domain::value_objects::{PostalCode as PostalCodeValue, Coordinates};
use crate::errors::AppResult;

/// Postal code repository interface
#[async_trait]
pub trait PostalCodeRepository: Send + Sync {
    /// Create a new postal code
    async fn create(&self, request: &CreatePostalCodeRequest, ward_id: Uuid) -> AppResult<PostalCode>;
    
    /// Find postal code by ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<PostalCode>>;
    
    /// Find postal code by code
    async fn find_by_code(&self, code: &PostalCodeValue) -> AppResult<Option<PostalCode>>;
    
    /// Find postal codes by ward ID
    async fn find_by_ward_id(&self, ward_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<PostalCode>>;
    
    /// Find postal codes near coordinates
    async fn find_near_coordinates(&self, coordinates: &Coordinates, radius_km: f64) -> AppResult<Vec<PostalCode>>;
    
    /// Update postal code
    async fn update(&self, id: Uuid, request: &UpdatePostalCodeRequest) -> AppResult<PostalCode>;
    
    /// Delete postal code
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Count postal codes by ward
    async fn count_by_ward(&self, ward_id: Uuid) -> AppResult<u64>;
    
    /// Search postal codes by code
    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<PostalCode>>;
}
