use async_trait::async_trait;

use crate::domain::entities::address::{Address, AddressValidationRequest, AddressValidationResponse};
use crate::errors::AppResult;

/// Address repository interface for complex address operations
#[async_trait]
pub trait AddressRepository: Send + Sync {
    /// Validate a complete address
    async fn validate_address(&self, request: &AddressValidationRequest) -> AppResult<AddressValidationResponse>;
    
    /// Find address by components
    async fn find_by_components(
        &self,
        state: &str,
        lga: &str,
        ward: &str,
        postal_code: &str,
    ) -> AppResult<Option<Address>>;
    
    /// Search for similar addresses
    async fn find_similar_addresses(&self, request: &AddressValidationRequest) -> AppResult<Vec<Address>>;
}
