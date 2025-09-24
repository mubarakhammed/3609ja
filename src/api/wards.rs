use axum::{extract::{Path, Query, State}, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{models::PaginationParams, queries::get_wards_by_lga};
use crate::errors::AppResult;

/// Get wards by LGA ID with pagination
pub async fn get_wards_by_lga_handler(
    State(pool): State<PgPool>,
    Path(lga_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);

    let result = get_wards_by_lga(&pool, lga_id, page, limit).await?;

    Ok(Json(serde_json::to_value(result)?))
}
