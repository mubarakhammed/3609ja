use serde::{Deserialize, Serialize};

use crate::domain::entities::address::{Address, AddressValidationRequest, AddressValidationResponse, AddressSuggestion};
use crate::application::dtos::{StateDto, LgaDto, WardDto, PostalCodeDto};

/// Address DTO for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDto {
    pub state: StateDto,
    pub lga: LgaDto,
    pub ward: WardDto,
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
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressValidationRequestDto {
    pub state: String,
    pub lga: String,
    pub ward: String,
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
#[derive(Debug, Serialize)]
pub struct AddressValidationResponseDto {
    pub valid: bool,
    pub canonical: Option<AddressDto>,
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
#[derive(Debug, Serialize)]
pub struct AddressSuggestionDto {
    pub state: Option<StateDto>,
    pub lga: Option<LgaDto>,
    pub ward: Option<WardDto>,
    pub postal_code: Option<PostalCodeDto>,
    pub reason: String,
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
