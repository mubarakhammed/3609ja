use axum::{extract::{Query, State}, Json};
use uuid::Uuid;

use crate::application::use_cases::state_use_cases::StateUseCases;
use crate::application::dtos::{StateDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// State controller
#[derive(Clone)]
pub struct StateController<SR: crate::domain::repositories::state_repository::StateRepository + Clone> {
    state_use_cases: StateUseCases<SR>,
}

impl<SR: crate::domain::repositories::state_repository::StateRepository + Clone> StateController<SR> {
    pub fn new(state_use_cases: StateUseCases<SR>) -> Self {
        Self { state_use_cases }
    }
}

/// Get all states with pagination
pub async fn get_states_handler<SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync>(
    State(controller): State<StateController<SR>>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<StateDto>>>
{
    let result = controller.state_use_cases.get_states(params).await?;
    Ok(Json(result))
}

/// Get state by ID
pub async fn get_state_by_id_handler<SR: crate::domain::repositories::state_repository::StateRepository + Clone + Send + Sync>(
    State(controller): State<StateController<SR>>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> AppResult<Json<Option<StateDto>>>
{
    let result = controller.state_use_cases.get_state_by_id(id).await?;
    Ok(Json(result))
}
