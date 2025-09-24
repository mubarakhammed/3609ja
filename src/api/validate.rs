use axum::{extract::State, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::db::{models::AddressValidationRequest, queries::validate_address};
use crate::errors::AppResult;

/// Validate a Nigerian address
pub async fn validate_address_handler(
    State(pool): State<PgPool>,
    Json(request): Json<AddressValidationRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // Validate the request
    request.validate()?;

    let result = validate_address(&pool, &request).await?;

    Ok(Json(serde_json::to_value(result)?))
}
