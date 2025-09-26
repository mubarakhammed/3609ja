use crate::application::dtos::address_dto::{AddressDto, AddressValidationResponseDto};
use crate::domain::entities::address::AddressValidationRequest;
use crate::domain::repositories::address_repository::AddressRepository;
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
    pub async fn validate_address(
        &self,
        request: AddressValidationRequest,
    ) -> AppResult<AddressValidationResponseDto> {
        let response = self.address_repository.validate_address(&request).await?;
        Ok(response.into())
    }

    /// Find address by components
    pub async fn find_address_by_components(
        &self,
        state: &str,
        lga: &str,
        ward: &str,
        postal_code: &str,
    ) -> AppResult<Option<AddressDto>> {
        let address = self
            .address_repository
            .find_by_components(state, lga, ward, postal_code)
            .await?;
        Ok(address.map(|a| a.into()))
    }

    /// Find similar addresses
    pub async fn find_similar_addresses(
        &self,
        request: AddressValidationRequest,
    ) -> AppResult<Vec<AddressDto>> {
        let addresses = self
            .address_repository
            .find_similar_addresses(&request)
            .await?;
        Ok(addresses.into_iter().map(|a| a.into()).collect())
    }
}
