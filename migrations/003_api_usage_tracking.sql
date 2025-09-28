-- API Usage Tracking Migration
-- This migration creates tables for tracking API usage and analytics

-- Create api_usage table
CREATE TABLE IF NOT EXISTS api_usage (
    id UUID PRIMARY KEY,
    endpoint VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    user_agent TEXT,
    ip_address INET NOT NULL,
    response_status SMALLINT NOT NULL,
    response_time_ms INTEGER NOT NULL,
    request_size_bytes INTEGER,
    response_size_bytes INTEGER,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    api_key VARCHAR(255),
    user_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_api_usage_timestamp ON api_usage(timestamp);
CREATE INDEX IF NOT EXISTS idx_api_usage_endpoint ON api_usage(endpoint);
CREATE INDEX IF NOT EXISTS idx_api_usage_status ON api_usage(response_status);
CREATE INDEX IF NOT EXISTS idx_api_usage_ip ON api_usage(ip_address);
CREATE INDEX IF NOT EXISTS idx_api_usage_api_key ON api_usage(api_key) WHERE api_key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_api_usage_user_id ON api_usage(user_id) WHERE user_id IS NOT NULL;

-- Create composite indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_api_usage_endpoint_timestamp ON api_usage(endpoint, timestamp);
CREATE INDEX IF NOT EXISTS idx_api_usage_status_timestamp ON api_usage(response_status, timestamp);

-- Create hourly usage summary materialized view for faster analytics
CREATE MATERIALIZED VIEW IF NOT EXISTS api_usage_hourly AS
SELECT 
    date_trunc('hour', timestamp) as hour,
    endpoint,
    COUNT(*) as request_count,
    COUNT(*) FILTER (WHERE response_status < 400) as successful_requests,
    COUNT(*) FILTER (WHERE response_status >= 400) as failed_requests,
    AVG(response_time_ms) as avg_response_time_ms,
    MIN(response_time_ms) as min_response_time_ms,
    MAX(response_time_ms) as max_response_time_ms
FROM api_usage
GROUP BY date_trunc('hour', timestamp), endpoint;

-- Create unique index on materialized view
CREATE UNIQUE INDEX IF NOT EXISTS idx_api_usage_hourly_hour_endpoint 
ON api_usage_hourly(hour, endpoint);

-- Create function to refresh hourly stats
CREATE OR REPLACE FUNCTION refresh_api_usage_hourly()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY api_usage_hourly;
END;
$$ LANGUAGE plpgsql;

-- Create daily usage summary materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS api_usage_daily AS
SELECT 
    date_trunc('day', timestamp) as day,
    endpoint,
    COUNT(*) as request_count,
    COUNT(*) FILTER (WHERE response_status < 400) as successful_requests,
    COUNT(*) FILTER (WHERE response_status >= 400) as failed_requests,
    AVG(response_time_ms) as avg_response_time_ms,
    COUNT(DISTINCT ip_address) as unique_ips
FROM api_usage
GROUP BY date_trunc('day', timestamp), endpoint;

-- Create unique index on daily materialized view
CREATE UNIQUE INDEX IF NOT EXISTS idx_api_usage_daily_day_endpoint 
ON api_usage_daily(day, endpoint);

-- Create function to refresh daily stats
CREATE OR REPLACE FUNCTION refresh_api_usage_daily()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY api_usage_daily;
END;
$$ LANGUAGE plpgsql;

COMMENT ON TABLE api_usage IS 'Tracks all API requests and responses for analytics and monitoring';
COMMENT ON MATERIALIZED VIEW api_usage_hourly IS 'Hourly aggregated API usage statistics for performance dashboards';
COMMENT ON MATERIALIZED VIEW api_usage_daily IS 'Daily aggregated API usage statistics for reporting';