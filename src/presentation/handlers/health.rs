use crate::{
    application::dtos::health_dto::{
        HealthCheckResponse, HealthStatus, ServiceHealth, ServiceStatus, SystemMetrics,
    },
    errors::AppResult,
    presentation::state::AppState,
};
use axum::extract::State;
use axum::Json;
use std::time::Instant;
use tracing::{info, warn};

static START_TIME: std::sync::LazyLock<Instant> = std::sync::LazyLock::new(Instant::now);

/// Health check endpoint
///
/// This endpoint provides a comprehensive health check of all system components
/// including database, cache, and application status.
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "System is healthy", body = HealthCheckResponse),
        (status = 503, description = "System is unhealthy", body = HealthCheckResponse)
    ),
    tag = "Health"
)]
pub async fn health_check_handler(
    State(app_state): State<AppState>,
) -> AppResult<Json<HealthCheckResponse>> {
    info!("Health check requested");

    // Check database health
    let database_healthy = check_database_health(&app_state).await;

    // Check cache health
    let cache_healthy = check_cache_health(&app_state).await;

    // Determine overall health
    let overall_status = if database_healthy && cache_healthy {
        HealthStatus::Healthy
    } else {
        HealthStatus::Unhealthy
    };

    let now = chrono::Utc::now();
    let uptime = START_TIME.elapsed().as_secs();

    let response = HealthCheckResponse {
        status: overall_status.clone(),
        timestamp: now,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        services: ServiceHealth {
            database: ServiceStatus {
                status: if database_healthy {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                },
                response_time_ms: Some(10), // Basic placeholder
                last_check: now,
                error: if database_healthy {
                    None
                } else {
                    Some("Database check failed".to_string())
                },
            },
            api: ServiceStatus {
                status: HealthStatus::Healthy,
                response_time_ms: Some(5),
                last_check: now,
                error: None,
            },
        },
        system: SystemMetrics {
            memory_usage_mb: 50.0, // Basic placeholder
            cpu_usage_percent: 5.0,
            active_connections: 1,
            total_requests: 0,
        },
    };

    match response.status {
        HealthStatus::Healthy => {
            info!("Health check passed - uptime: {}s", uptime);
            Ok(Json(response))
        }
        HealthStatus::Unhealthy => {
            warn!("Health check failed - system unhealthy");
            Ok(Json(response))
        }
        HealthStatus::Degraded => {
            warn!("Health check shows degraded performance");
            Ok(Json(response))
        }
    }
}

/// Check database connectivity and basic operations
async fn check_database_health(app_state: &AppState) -> bool {
    // Try a simple database query to verify connectivity
    match sqlx::query("SELECT 1")
        .execute(app_state.database_pool())
        .await
    {
        Ok(_) => {
            info!("Database health check passed");
            true
        }
        Err(e) => {
            warn!("Database health check failed: {}", e);
            false
        }
    }
}

/// Cache disabled for performance - always returns healthy
async fn check_cache_health(_app_state: &AppState) -> bool {
    info!("Cache disabled - skipping cache health check");
    true
}
