use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::application::dtos::{
    address_dto::{AddressDto, AddressValidationRequestDto, AddressValidationResponseDto},
    LgaDto, PaginatedResponse, PaginationParams, PostalCodeDto, StateDto, WardDto,
};
use crate::application::use_cases::search_use_cases::SearchResultDto;
use crate::domain::entities::address::AddressValidationRequest;
use crate::errors::AppResult;
use crate::presentation::state::AppState;

// State handlers
pub async fn get_states_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<StateDto>>> {
    let result = app_state.state_use_cases.get_states(params).await?;
    Ok(Json(result))
}

pub async fn get_state_by_id_handler(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<StateDto>>> {
    let result = app_state.state_use_cases.get_state_by_id(id).await?;
    Ok(Json(result))
}

// LGA handlers
pub async fn get_lgas_by_state_handler(
    State(app_state): State<AppState>,
    Path(state_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<LgaDto>>> {
    let result = app_state
        .lga_use_cases
        .get_lgas_by_state(state_id, params)
        .await?;
    Ok(Json(result))
}

pub async fn get_lga_by_id_handler(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<LgaDto>>> {
    let result = app_state.lga_use_cases.get_lga_by_id(id).await?;
    Ok(Json(result))
}

// Ward handlers
pub async fn get_wards_by_lga_handler(
    State(app_state): State<AppState>,
    Path(lga_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<WardDto>>> {
    let result = app_state
        .ward_use_cases
        .get_wards_by_lga(lga_id, params)
        .await?;
    Ok(Json(result))
}

pub async fn get_ward_by_id_handler(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<WardDto>>> {
    let result = app_state.ward_use_cases.get_ward_by_id(id).await?;
    Ok(Json(result))
}

// Postal code handlers
pub async fn get_postal_codes_by_ward_handler(
    State(app_state): State<AppState>,
    Path(ward_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<PostalCodeDto>>> {
    let result = app_state
        .postal_code_use_cases
        .get_postal_codes_by_ward(ward_id, params)
        .await?;
    Ok(Json(result))
}

pub async fn get_postal_code_by_id_handler(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<PostalCodeDto>>> {
    let result = app_state
        .postal_code_use_cases
        .get_postal_code_by_id(id)
        .await?;
    Ok(Json(result))
}

pub async fn get_postal_code_by_code_handler(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Json<Option<PostalCodeDto>>> {
    let result = app_state
        .postal_code_use_cases
        .get_postal_code_by_code(&code)
        .await?;
    Ok(Json(result))
}

#[derive(Debug, serde::Deserialize)]
pub struct NearbyParams {
    pub lat: f64,
    pub lng: f64,
    pub radius_km: Option<f64>,
}

pub async fn find_nearby_postal_codes_handler(
    State(app_state): State<AppState>,
    Query(params): Query<NearbyParams>,
) -> AppResult<Json<Vec<PostalCodeDto>>> {
    let coordinates = crate::domain::value_objects::Coordinates::new(params.lat, params.lng)?;
    let radius = params.radius_km.unwrap_or(10.0);
    let result = app_state
        .postal_code_use_cases
        .find_near_coordinates(coordinates, radius)
        .await?;
    Ok(Json(result))
}

// Address handlers
pub async fn validate_address_handler(
    State(app_state): State<AppState>,
    Json(request): Json<AddressValidationRequestDto>,
) -> AppResult<Json<AddressValidationResponseDto>> {
    let validation_request = AddressValidationRequest {
        state: request.state,
        lga: request.lga,
        ward: request.ward,
        postal_code: request.postal_code,
    };

    let result = app_state
        .address_use_cases
        .validate_address(validation_request)
        .await?;
    Ok(Json(result))
}

#[derive(Debug, serde::Deserialize)]
pub struct AddressComponentsParams {
    pub state: String,
    pub lga: String,
    pub ward: String,
    pub postal_code: String,
}

pub async fn find_address_by_components_handler(
    State(app_state): State<AppState>,
    Query(params): Query<AddressComponentsParams>,
) -> AppResult<Json<Option<AddressDto>>> {
    let result = app_state
        .address_use_cases
        .find_address_by_components(
            &params.state,
            &params.lga,
            &params.ward,
            &params.postal_code,
        )
        .await?;
    Ok(Json(result))
}

pub async fn find_similar_addresses_handler(
    State(app_state): State<AppState>,
    Json(request): Json<AddressValidationRequestDto>,
) -> AppResult<Json<Vec<AddressDto>>> {
    let validation_request = AddressValidationRequest {
        state: request.state,
        lga: request.lga,
        ward: request.ward,
        postal_code: request.postal_code,
    };

    let result = app_state
        .address_use_cases
        .find_similar_addresses(validation_request)
        .await?;
    Ok(Json(result))
}

// Search handlers
#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
}

// Search handlers
pub async fn search_all_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<SearchResultDto>> {
    let result = app_state
        .search_use_cases
        .search_all(&search_params.query, params)
        .await?;
    Ok(Json(result))
}

pub async fn search_states_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<StateDto>>> {
    let result = app_state
        .search_use_cases
        .search_states(&search_params.query, params)
        .await?;
    Ok(Json(result))
}

pub async fn search_lgas_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<LgaDto>>> {
    let result = app_state
        .search_use_cases
        .search_lgas(&search_params.query, params)
        .await?;
    Ok(Json(result))
}

pub async fn search_wards_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<WardDto>>> {
    let result = app_state
        .search_use_cases
        .search_wards(&search_params.query, params)
        .await?;
    Ok(Json(result))
}

pub async fn search_postal_codes_handler(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<PostalCodeDto>>> {
    let result = app_state
        .search_use_cases
        .search_postal_codes(&search_params.query, params)
        .await?;
    Ok(Json(result))
}
