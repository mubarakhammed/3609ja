use axum::{extract::{Query, State}, Json};

use crate::application::use_cases::search_use_cases::{SearchUseCases, SearchResultDto};
use crate::application::dtos::{StateDto, LgaDto, WardDto, PostalCodeDto, PaginationParams};
use crate::errors::AppResult;

/// Search controller
#[derive(Clone)]
pub struct SearchController<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone,
> {
    search_use_cases: SearchUseCases<SR, LR, WR, PR>,
}

impl<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone,
> SearchController<SR, LR, WR, PR> {
    pub fn new(search_use_cases: SearchUseCases<SR, LR, WR, PR>) -> Self {
        Self { search_use_cases }
    }
}

/// Search across all entities
pub async fn search_all_handler<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync,
>(
    State(controller): State<SearchController<SR, LR, WR, PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<SearchResultDto>>
{
    let result = controller.search_use_cases.search_all(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search states only
pub async fn search_states_handler<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync,
>(
    State(controller): State<SearchController<SR, LR, WR, PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<StateDto>>>
{
    let result = controller.search_use_cases.search_states(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search LGAs only
pub async fn search_lgas_handler<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync,
>(
    State(controller): State<SearchController<SR, LR, WR, PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<LgaDto>>>
{
    let result = controller.search_use_cases.search_lgas(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search wards only
pub async fn search_wards_handler<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync,
>(
    State(controller): State<SearchController<SR, LR, WR, PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<WardDto>>>
{
    let result = controller.search_use_cases.search_wards(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search postal codes only
pub async fn search_postal_codes_handler<
    SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync,
    LR: crate::domain::repositories::lga_repository::LgaRepository + Clone + Send + Sync,
    WR: crate::domain::repositories::ward_repository::WardRepository + Clone + Send + Sync,
    PR: crate::domain::repositories::postal_code_repository::PostalCodeRepository + Clone + Send + Sync,
>(
    State(controller): State<SearchController<SR, LR, WR, PR>>,
    Query(params): Query<PaginationParams>,
    axum::extract::Query(search_params): axum::extract::Query<SearchParams>,
) -> AppResult<Json<Vec<PostalCodeDto>>>
{
    let result = controller.search_use_cases.search_postal_codes(&search_params.query, params).await?;
    Ok(Json(result))
}

/// Search parameters
#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
}