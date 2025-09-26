use axum::{extract::{Query, State, Path}, Json};
use uuid::Uuid;

use crate::application::use_cases::postal_code_use_cases::PostalCodeUseCases;
use crate::application::dtos::{PostalCodeDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// Postal code controller
#[derive(Clone)]
pub struct PostalCodeController<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone> {
    postal_code_use_cases: PostalCodeUseCases<PR>,
}

impl<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone> PostalCodeController<PR> {
    pub fn new(postal_code_use_cases: PostalCodeUseCases<PR>) -> Self {
        Self { postal_code_use_cases }
    }
}

/// Get postal codes by ward ID with pagination
pub async fn get_postal_codes_by_ward_handler<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync>(
    State(controller): State<PostalCodeController<PR>>,
    Path(ward_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<PostalCodeDto>>>
{
    let result = controller.postal_code_use_cases.get_postal_codes_by_ward(ward_id, params).await?;
    Ok(Json(result))
}

/// Get postal code by ID
pub async fn get_postal_code_by_id_handler<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync>(
    State(controller): State<PostalCodeController<PR>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<PostalCodeDto>>>
{
    let result = controller.postal_code_use_cases.get_postal_code_by_id(id).await?;
    Ok(Json(result))
}

/// Get postal code by code
pub async fn get_postal_code_by_code_handler<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync>(
    State(controller): State<PostalCodeController<PR>>,
    Path(code): Path<String>,
) -> AppResult<Json<Option<PostalCodeDto>>>
{
    let result = controller.postal_code_use_cases.get_postal_code_by_code(&code).await?;
    Ok(Json(result))
}

/// Find nearby postal codes
pub async fn find_nearby_postal_codes_handler<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync>(
    State(controller): State<PostalCodeController<PR>>,
    Query(params): Query<NearbyParams>,
) -> AppResult<Json<Vec<PostalCodeDto>>>
{
    let result = controller.postal_code_use_cases.find_nearby_postal_codes(
        params.lat,
        params.lng,
        params.radius_km.unwrap_or(10.0)
    ).await?;
    Ok(Json(result))
}

/// Search postal codes
pub async fn search_postal_codes_handler<PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync>(
    State(controller): State<PostalCodeController<PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<PostalCodeDto>>>
{
    let result = controller.postal_code_use_cases.search_postal_codes(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search parameters
#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
}

/// Nearby search parameters
#[derive(serde::Deserialize)]
pub struct NearbyParams {
    pub lat: f64,
    pub lng: f64,
    pub radius_km: Option<f64>,
}
