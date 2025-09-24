use axum::{extract::{Path, Query, State}, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{models::PaginationParams, queries::get_lgas_by_state};
use crate::errors::AppResult;

/// Get LGAs by state ID with pagination
pub async fn get_lgas_by_state_handler(
    State(pool): State<PgPool>,
    Path(state_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    let result = get_lgas_by_state(&pool, state_id, page, limit).await?;

    Ok(Json(serde_json::to_value(result)?))
}
