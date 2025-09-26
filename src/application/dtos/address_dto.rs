use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::address::{Address, AddressValidationRequest, AddressValidationResponse, AddressSuggestion};
use crate::application::dtos::{StateDto, LgaDto, WardDto, PostalCodeDto};

/// Address DTO for API responses
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddressDto {
    /// State information
    pub state: StateDto,
    /// Local Government Area information
    pub lga: LgaDto,
    /// Ward information
    pub ward: WardDto,
    /// Postal code information
    pub postal_code: PostalCodeDto,
}

impl From<Address> for AddressDto {
    fn from(address: Address) -> Self {
        Self {
            state: address.state.into(),
            lga: address.lga.into(),
            ward: address.ward.into(),
            postal_code: address.postal_code.into(),
        }
    }
}

/// Address validation request DTO
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AddressValidationRequestDto {
    /// State name
    #[schema(example = "Lagos")]
    pub state: String,
    /// Local Government Area name
    #[schema(example = "Ikeja")]
    pub lga: String,
    /// Ward name
    #[schema(example = "Ikeja")]
    pub ward: String,
    /// Postal code
    #[schema(example = "100001")]
    pub postal_code: String,
}

impl From<AddressValidationRequestDto> for AddressValidationRequest {
    fn from(dto: AddressValidationRequestDto) -> Self {
        Self {
            state: dto.state,
            lga: dto.lga,
            ward: dto.ward,
            postal_code: dto.postal_code,
        }
    }
}

/// Address validation response DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct AddressValidationResponseDto {
    /// Whether the address is valid
    #[schema(example = true)]
    pub valid: bool,
    /// Canonical address if valid
    pub canonical: Option<AddressDto>,
    /// Suggested corrections if invalid
    pub suggestions: Vec<AddressSuggestionDto>,
}

impl From<AddressValidationResponse> for AddressValidationResponseDto {
    fn from(response: AddressValidationResponse) -> Self {
        Self {
            valid: response.valid,
            canonical: response.canonical.map(|addr| addr.into()),
            suggestions: response.suggestions.into_iter().map(|s| s.into()).collect(),
        }
    }
}

/// Address suggestion DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct AddressSuggestionDto {
    /// Suggested state
    pub state: Option<StateDto>,
    /// Suggested LGA
    pub lga: Option<LgaDto>,
    /// Suggested ward
    pub ward: Option<WardDto>,
    /// Suggested postal code
    pub postal_code: Option<PostalCodeDto>,
    /// Reason for suggestion
    #[schema(example = "Similar name found")]
    pub reason: String,
    /// Confidence score (0.0 to 1.0)
    #[schema(example = 0.85, minimum = 0.0, maximum = 1.0)]
    pub confidence: f64,
}

impl From<AddressSuggestion> for AddressSuggestionDto {
    fn from(suggestion: AddressSuggestion) -> Self {
        Self {
            state: suggestion.state.map(|s| s.into()),
            lga: suggestion.lga.map(|l| l.into()),
            ward: suggestion.ward.map(|w| w.into()),
            postal_code: suggestion.postal_code.map(|p| p.into()),
            reason: suggestion.reason,
            confidence: suggestion.confidence,
        }
    }
}
