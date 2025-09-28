use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// API usage tracking entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiUsage {
    pub id: Uuid,
    pub endpoint: String,
    pub method: String,
    pub user_agent: Option<String>,
    pub ip_address: String,
    pub response_status: u16,
    pub response_time_ms: u32,
    pub request_size_bytes: Option<u32>,
    pub response_size_bytes: Option<u32>,
    pub timestamp: DateTime<Utc>,
    pub api_key: Option<String>,
    pub user_id: Option<String>,
}

impl ApiUsage {
    /// Create a new API usage record
    pub fn new(
        endpoint: String,
        method: String,
        user_agent: Option<String>,
        ip_address: String,
        response_status: u16,
        response_time_ms: u32,
        request_size_bytes: Option<u32>,
        response_size_bytes: Option<u32>,
        api_key: Option<String>,
        user_id: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            endpoint,
            method,
            user_agent,
            ip_address,
            response_status,
            response_time_ms,
            request_size_bytes,
            response_size_bytes,
            timestamp: Utc::now(),
            api_key,
            user_id,
        }
    }
}

/// Usage statistics aggregated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub average_response_time_ms: f64,
    pub top_endpoints: Vec<EndpointStats>,
    pub requests_by_hour: Vec<HourlyStats>,
    pub status_code_distribution: Vec<StatusCodeStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStats {
    pub endpoint: String,
    pub request_count: i64,
    pub average_response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyStats {
    pub hour: DateTime<Utc>,
    pub request_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCodeStats {
    pub status_code: u16,
    pub count: i64,
}
