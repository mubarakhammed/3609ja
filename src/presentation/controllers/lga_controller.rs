use axum::{extract::{Query, State, Path}, Json};
use uuid::Uuid;

use crate::application::use_cases::lga_use_cases::LgaUseCases;
use crate::application::dtos::{LgaDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// LGA controller
#[derive(Clone)]
pub struct LgaController<LR: crate::domain::repositories::lga_repository::LgaRepository + Clone> {
    lga_use_cases: LgaUseCases<LR>,
}

impl<LR: crate::domain::repositories::lga_repository::LgaRepository + Clone> LgaController<LR> {
    pub fn new(lga_use_cases: LgaUseCases<LR>) -> Self {
        Self { lga_use_cases }
    }
}

/// Get LGAs by state ID with pagination
pub async fn get_lgas_by_state_handler<LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync>(
    State(controller): State<LgaController<LR>>,
    Path(state_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<LgaDto>>>
{
    let result = controller.lga_use_cases.get_lgas_by_state(state_id, params).await?;
    Ok(Json(result))
}

/// Get LGA by ID
pub async fn get_lga_by_id_handler<LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync>(
    State(controller): State<LgaController<LR>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Option<LgaDto>>>
{
    let result = controller.lga_use_cases.get_lga_by_id(id).await?;
    Ok(Json(result))
}

/// Search LGAs
pub async fn search_lgas_handler<LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync>(
    State(controller): State<LgaController<LR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<LgaDto>>>
{
    let result = controller.lga_use_cases.search_lgas(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search parameters
#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
}
