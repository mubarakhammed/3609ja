use utoipa::OpenApi;

use crate::application::dtos::{
    StateDto, LgaDto, WardDto, PostalCodeDto, PaginatedResponse, PaginationParams,
    address_dto::{AddressValidationRequestDto, AddressValidationResponseDto, AddressDto, AddressSuggestionDto},
};
use crate::application::use_cases::search_use_cases::SearchResultDto;
use crate::errors::AppError;
use crate::presentation::handlers::{
    get_states_handler, get_state_by_id_handler, validate_address_handler,
};

/// OpenAPI documentation for Nigeria Geo API
#[derive(OpenApi)]
#[openapi(
    paths(
        // States
        get_states_handler,
        get_state_by_id_handler,
        
        // Address Validation
        validate_address_handler,
    ),
    components(
        schemas(
            // Core DTOs
            StateDto,
            LgaDto,
            WardDto,
            PostalCodeDto,
            AddressDto,
            
            // Pagination
            PaginatedResponse<StateDto>,
            PaginatedResponse<LgaDto>,
            PaginatedResponse<WardDto>,
            PaginatedResponse<PostalCodeDto>,
            PaginationParams,
            crate::application::dtos::pagination_dto::PaginationMeta,
            
            // Address Validation
            AddressValidationRequestDto,
            AddressValidationResponseDto,
            AddressSuggestionDto,
            
            // Search
            SearchResultDto,
            
            // Errors
            AppError,
        )
    ),
    tags(
        (name = "States", description = "Nigerian states management"),
        (name = "LGAs", description = "Local Government Areas management"),
        (name = "Wards", description = "Wards management"),
        (name = "Postal Codes", description = "Postal codes and geographic data"),
        (name = "Address Validation", description = "Address validation and suggestions"),
        (name = "Search", description = "Search across all geographic entities"),
    ),
    info(
        title = "Nigeria Geo + Postal + Validation API",
        description = "A comprehensive API for Nigerian geographic data, postal codes, and address validation",
        version = "1.0.0",
        contact(
            name = "Nigeria Geo API",
            email = "support@nigeriageoapi.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
        (url = "https://api.nigeriageo.com", description = "Production server")
    )
)]
pub struct ApiDoc;
