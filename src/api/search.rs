use axum::{extract::{Query, State}, Json};
use sqlx::PgPool;

use crate::db::{models::PaginationParams, queries::search_all};
use crate::errors::AppResult;

/// Search across all entities
pub async fn search_handler(
    State(pool): State<PgPool>,
    Query(params): Query<SearchParams>,
) -> AppResult<Json<serde_json::Value>> {
    let page = params.pagination.page.unwrap_or(1);
    let limit = params.pagination.limit.unwrap_or(20);

    if params.query.is_empty() {
        return Err(crate::errors::AppError::Validation(
            validator::ValidationErrors::new()
        ));
    }

    let result = search_all(&pool, &params.query, page, limit).await?;

    Ok(Json(serde_json::to_value(result)?))
}

#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub query: String,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}
