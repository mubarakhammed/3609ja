use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

use crate::domain::entities::api_usage::{UsageStats, EndpointStats, HourlyStats, StatusCodeStats};
use crate::domain::repositories::api_usage_repository::ApiUsageRepository;
use crate::errors::AppResult;
use crate::presentation::state::AppState;

#[derive(Debug, Deserialize)]
pub struct UsageStatsQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub period_hours: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TopEndpointsQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct IpUsageQuery {
    pub ip_address: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyUsageQuery {
    pub api_key: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ApiUsageResponse<T> {
    pub data: T,
    pub period: UsagePeriod,
}

#[derive(Debug, Serialize)]
pub struct UsagePeriod {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Get general usage statistics
pub async fn get_usage_stats_handler(
    State(app_state): State<AppState>,
    Query(params): Query<UsageStatsQuery>,
) -> AppResult<Json<ApiUsageResponse<UsageStats>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| {
        let hours = params.period_hours.unwrap_or(24);
        end_date - Duration::hours(hours)
    });

    let stats = app_state
        .api_usage_repository
        .get_usage_stats(start_date, end_date)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: stats,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Get top endpoints by request count
pub async fn get_top_endpoints_handler(
    State(app_state): State<AppState>,
    Query(params): Query<TopEndpointsQuery>,
) -> AppResult<Json<ApiUsageResponse<Vec<EndpointStats>>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| end_date - Duration::hours(24));
    let limit = params.limit.unwrap_or(10);

    let endpoints = app_state
        .api_usage_repository
        .get_top_endpoints(start_date, end_date, limit)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: endpoints,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Get hourly usage statistics
pub async fn get_hourly_stats_handler(
    State(app_state): State<AppState>,
    Query(params): Query<UsageStatsQuery>,
) -> AppResult<Json<ApiUsageResponse<Vec<HourlyStats>>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| {
        let hours = params.period_hours.unwrap_or(24);
        end_date - Duration::hours(hours)
    });

    let stats = app_state
        .api_usage_repository
        .get_hourly_stats(start_date, end_date)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: stats,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Get status code distribution
pub async fn get_status_code_stats_handler(
    State(app_state): State<AppState>,
    Query(params): Query<UsageStatsQuery>,
) -> AppResult<Json<ApiUsageResponse<Vec<StatusCodeStats>>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| {
        let hours = params.period_hours.unwrap_or(24);
        end_date - Duration::hours(hours)
    });

    let stats = app_state
        .api_usage_repository
        .get_status_code_stats(start_date, end_date)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: stats,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Get usage by IP address
pub async fn get_usage_by_ip_handler(
    State(app_state): State<AppState>,
    Query(params): Query<IpUsageQuery>,
) -> AppResult<Json<ApiUsageResponse<Vec<crate::domain::entities::api_usage::ApiUsage>>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| end_date - Duration::hours(24));

    let usage = app_state
        .api_usage_repository
        .get_usage_by_ip(&params.ip_address, start_date, end_date)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: usage,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Get usage by API key
pub async fn get_usage_by_api_key_handler(
    State(app_state): State<AppState>,
    Query(params): Query<ApiKeyUsageQuery>,
) -> AppResult<Json<ApiUsageResponse<Vec<crate::domain::entities::api_usage::ApiUsage>>>> {
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| end_date - Duration::hours(24));

    let usage = app_state
        .api_usage_repository
        .get_usage_by_api_key(&params.api_key, start_date, end_date)
        .await?;

    Ok(Json(ApiUsageResponse {
        data: usage,
        period: UsagePeriod { start_date, end_date },
    }))
}

/// Refresh materialized views for better performance
pub async fn refresh_stats_views_handler(
    State(app_state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    app_state.api_usage_repository.refresh_stats_views().await?;

    Ok(Json(serde_json::json!({
        "message": "Stats views refreshed successfully",
        "timestamp": Utc::now()
    })))
}

/// Clean up old usage records
#[derive(Debug, Deserialize)]
pub struct CleanupQuery {
    pub days_to_keep: Option<i64>,
}

pub async fn cleanup_old_records_handler(
    State(app_state): State<AppState>,
    Query(params): Query<CleanupQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let days_to_keep = params.days_to_keep.unwrap_or(90); // Default: keep 90 days
    let cutoff_date = Utc::now() - Duration::days(days_to_keep);

    let deleted_count = app_state
        .api_usage_repository
        .cleanup_old_records(cutoff_date)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Old records cleaned up successfully",
        "deleted_count": deleted_count,
        "cutoff_date": cutoff_date,
        "timestamp": Utc::now()
    })))
}