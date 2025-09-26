use axum::{extract::{Query, State}, Json};

use crate::application::use_cases::address_use_cases::AddressUseCases;
use crate::application::dtos::address_dto::{AddressValidationRequestDto, AddressValidationResponseDto, AddressDto};
use crate::domain::entities::address::AddressValidationRequest;
use crate::errors::AppResult;

/// Address controller
pub struct AddressController<AR: crate::domain::repositories::address_repository::AddressRepository> {
    address_use_cases: AddressUseCases<AR>,
}

impl<AR: crate::domain::repositories::address_repository::AddressRepository> AddressController<AR> {
    pub fn new(address_use_cases: AddressUseCases<AR>) -> Self {
        Self { address_use_cases }
    }
}

/// Validate an address
pub async fn validate_address_handler<AR: crate::domain::repositories::address_repository::AddressRepository + Send + Sync>(
    State(controller): State<AddressController<AR>>,
    Json(request): Json<AddressValidationRequestDto>,
) -> AppResult<Json<AddressValidationResponseDto>>
{
    let validation_request = AddressValidationRequest {
        state: request.state,
        lga: request.lga,
        ward: request.ward,
        postal_code: request.postal_code,
    };

    let result = controller.address_use_cases.validate_address(validation_request).await?;
    Ok(Json(result))
}

/// Find address by components
pub async fn find_address_by_components_handler<AR: crate::domain::repositories::address_repository::AddressRepository + Send + Sync>(
    State(controller): State<AddressController<AR>>,
    Query(params): Query<AddressComponentsParams>,
) -> AppResult<Json<Option<AddressDto>>>
{
    let result = controller.address_use_cases.find_address_by_components(
        &params.state,
        &params.lga,
        &params.ward,
        &params.postal_code,
    ).await?;
    Ok(Json(result))
}

/// Find similar addresses
pub async fn find_similar_addresses_handler<AR: crate::domain::repositories::address_repository::AddressRepository + Send + Sync>(
    State(controller): State<AddressController<AR>>,
    Json(request): Json<AddressValidationRequestDto>,
) -> AppResult<Json<Vec<AddressDto>>>
{
    let validation_request = AddressValidationRequest {
        state: request.state,
        lga: request.lga,
        ward: request.ward,
        postal_code: request.postal_code,
    };

    let result = controller.address_use_cases.find_similar_addresses(validation_request).await?;
    Ok(Json(result))
}

/// Address components parameters
#[derive(serde::Deserialize)]
pub struct AddressComponentsParams {
    pub state: String,
    pub lga: String,
    pub ward: String,
    pub postal_code: String,
}