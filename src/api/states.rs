use axum::{extract::{Query, State}, Json};
use sqlx::PgPool;

use crate::db::{models::PaginationParams, queries::get_states};
use crate::errors::AppResult;

/// Get all states with pagination
pub async fn get_states_handler(
    State(pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    let result = get_states(&pool, page, limit).await?;

    Ok(Json(serde_json::to_value(result)?))
}
