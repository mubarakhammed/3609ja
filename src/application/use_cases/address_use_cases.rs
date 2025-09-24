use crate::domain::entities::address::AddressValidationRequest;
use crate::domain::repositories::address_repository::AddressRepository;
use crate::application::dtos::address_dto::AddressValidationResponseDto;
use crate::errors::AppResult;

/// Address use cases
pub struct AddressUseCases<R: AddressRepository> {
    address_repository: R,
}

impl<R: AddressRepository> AddressUseCases<R> {
    pub fn new(address_repository: R) -> Self {
        Self { address_repository }
    }

    /// Validate an address
    pub async fn validate_address(&self, request: AddressValidationRequest) -> AppResult<AddressValidationResponseDto> {
        let response = self.address_repository.validate_address(&request).await?;
        Ok(response.into())
    }
}
