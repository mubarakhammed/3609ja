use axum::{extract::{Query, State, Path}, Json};
use uuid::Uuid;

use crate::application::use_cases::ward_use_cases::WardUseCases;
use crate::application::dtos::{WardDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// Ward controller
#[derive(Clone)]
pub struct WardController<WR: crate::domain::repositories::ward_repository::WardRepository + Clone> {
    ward_use_cases: WardUseCases<WR>,
}

impl<WR: crate::domain::repositories::ward_repository::WardRepository + Clone> WardController<WR> {
    pub fn new(ward_use_cases: WardUseCases<WR>) -> Self {
        Self { ward_use_cases }
    }
}

/// Get wards by LGA ID with pagination
pub async fn get_wards_by_lga_handler<WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync>(
    State(controller): State<WardController<WR>>,
    Path(lga_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<WardDto>>>
{
    let result = controller.ward_use_cases.get_wards_by_lga(lga_id, params).await?;
    Ok(Json(result))
}

/// Get ward by ID
pub async fn get_ward_by_id_handler<WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync>(
    State(controller): State<WardController<WR>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<WardDto>>>
{
    let result = controller.ward_use_cases.get_ward_by_id(id).await?;
    Ok(Json(result))
}

/// Search wards
pub async fn search_wards_handler<WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync>(
    State(controller): State<WardController<WR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<WardDto>>>
{
    let result = controller.ward_use_cases.search_wards(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search parameters
#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
}
