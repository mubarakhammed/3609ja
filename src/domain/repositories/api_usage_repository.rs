use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::entities::api_usage::{
    ApiUsage, EndpointStats, HourlyStats, StatusCodeStats, UsageStats,
};
use crate::errors::AppResult;

#[async_trait]
pub trait ApiUsageRepository: Send + Sync {
    /// Log a new API usage record
    async fn log_usage(&self, usage: ApiUsage) -> AppResult<()>;

    /// Get usage statistics for a date range
    async fn get_usage_stats(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<UsageStats>;

    /// Get top endpoints by request count
    async fn get_top_endpoints(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        limit: u32,
    ) -> AppResult<Vec<EndpointStats>>;

    /// Get hourly request statistics
    async fn get_hourly_stats(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<Vec<HourlyStats>>;

    /// Get status code distribution
    async fn get_status_code_stats(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<Vec<StatusCodeStats>>;

    /// Get usage by IP address
    async fn get_usage_by_ip(
        &self,
        ip_address: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<Vec<ApiUsage>>;

    /// Get usage by API key
    async fn get_usage_by_api_key(
        &self,
        api_key: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> AppResult<Vec<ApiUsage>>;

    /// Clean up old usage records (for data retention)
    async fn cleanup_old_records(&self, older_than: DateTime<Utc>) -> AppResult<u64>;

    /// Refresh materialized views for better performance
    async fn refresh_stats_views(&self) -> AppResult<()>;
}
