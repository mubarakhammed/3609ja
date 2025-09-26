use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Health check response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: HealthStatus,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub uptime_seconds: u64,
    pub services: ServiceHealth,
    pub system: SystemMetrics,
}

/// Overall health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unhealthy")]
    Unhealthy,
}

/// Individual service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub database: ServiceStatus,
    pub api: ServiceStatus,
}

/// Service status details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub status: HealthStatus,
    pub response_time_ms: Option<u64>,
    pub last_check: DateTime<Utc>,
    pub error: Option<String>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_connections: u32,
    pub total_requests: u64,
}

impl HealthCheckResponse {
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Healthy,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: 0,
            services: ServiceHealth {
                database: ServiceStatus {
                    status: HealthStatus::Healthy,
                    response_time_ms: None,
                    last_check: Utc::now(),
                    error: None,
                },
                api: ServiceStatus {
                    status: HealthStatus::Healthy,
                    response_time_ms: None,
                    last_check: Utc::now(),
                    error: None,
                },
            },
            system: SystemMetrics {
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                active_connections: 0,
                total_requests: 0,
            },
        }
    }

    pub fn with_uptime(mut self, uptime_seconds: u64) -> Self {
        self.uptime_seconds = uptime_seconds;
        self
    }

    pub fn with_database_status(mut self, status: ServiceStatus) -> Self {
        self.services.database = status;
        self.update_overall_status();
        self
    }

    pub fn with_system_metrics(mut self, metrics: SystemMetrics) -> Self {
        self.system = metrics;
        self
    }

    fn update_overall_status(&mut self) {
        let db_status = &self.services.database.status;
        let api_status = &self.services.api.status;

        self.status = match (db_status, api_status) {
            (HealthStatus::Healthy, HealthStatus::Healthy) => HealthStatus::Healthy,
            (HealthStatus::Unhealthy, _) | (_, HealthStatus::Unhealthy) => HealthStatus::Unhealthy,
            _ => HealthStatus::Degraded,
        };
    }
}

impl Default for HealthCheckResponse {
    fn default() -> Self {
        Self::new()
    }
}
