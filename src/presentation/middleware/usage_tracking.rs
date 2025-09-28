use axum::{
    extract::{ConnectInfo, Request},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tokio::sync::mpsc;

use crate::domain::entities::api_usage::ApiUsage;
use crate::domain::repositories::api_usage_repository::ApiUsageRepository;

/// Middleware for tracking API usage
pub async fn track_usage_middleware(
    connect_info: Option<ConnectInfo<SocketAddr>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().to_string();
    let uri = request.uri().path().to_string();
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let api_key = headers
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    let user_id = headers
        .get("x-user-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get request size
    let request_size = headers
        .get("content-length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());

    // Process the request
    let response = next.run(request).await;

    // Calculate response time
    let response_time = start_time.elapsed().as_millis() as u32;
    let status_code = response.status().as_u16();

    // Get response size (estimate)
    let response_size = response
        .headers()
        .get("content-length")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());

    // Extract IP address from ConnectInfo or use fallback
    let ip_address = connect_info
        .map(|ConnectInfo(addr)| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Create usage record
    let usage = ApiUsage::new(
        uri,
        method,
        user_agent,
        ip_address,
        status_code,
        response_time,
        request_size,
        response_size,
        api_key,
        user_id,
    );

    // Log usage asynchronously (don't block response)
    tokio::spawn(async move {
        if let Err(e) = log_usage_async(usage).await {
            tracing::warn!("Failed to log API usage: {}", e);
        }
    });

    response
}

/// Async function to log usage (to be called in a separate task)
async fn log_usage_async(usage: ApiUsage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This would need access to the repository
    // For now, we'll just log it
    tracing::info!(
        "API Usage: {} {} - Status: {} - Time: {}ms - IP: {}",
        usage.method,
        usage.endpoint,
        usage.response_status,
        usage.response_time_ms,
        usage.ip_address
    );
    Ok(())
}

/// Background service for batch processing usage logs
pub struct UsageTracker {
    sender: mpsc::UnboundedSender<ApiUsage>,
}

impl UsageTracker {
    pub fn new(repository: Arc<dyn ApiUsageRepository>) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<ApiUsage>();

        // Spawn background task to process usage logs
        tokio::spawn(async move {
            let mut batch = Vec::new();
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

            loop {
                tokio::select! {
                    // Collect usage logs
                    usage = receiver.recv() => {
                        match usage {
                            Some(usage) => {
                                batch.push(usage);

                                // Process batch when it reaches 100 items
                                if batch.len() >= 100 {
                                    if let Err(e) = process_batch(&repository, &mut batch).await {
                                        tracing::error!("Failed to process usage batch: {}", e);
                                    }
                                }
                            }
                            None => break, // Channel closed
                        }
                    }

                    // Process remaining items every 5 seconds
                    _ = interval.tick() => {
                        if !batch.is_empty() {
                            if let Err(e) = process_batch(&repository, &mut batch).await {
                                tracing::error!("Failed to process usage batch: {}", e);
                            }
                        }
                    }
                }
            }
        });

        Self { sender }
    }

    pub fn track(&self, usage: ApiUsage) {
        if let Err(e) = self.sender.send(usage) {
            tracing::warn!("Failed to queue usage tracking: {}", e);
        }
    }
}

/// Process a batch of usage logs
async fn process_batch(
    repository: &Arc<dyn ApiUsageRepository>,
    batch: &mut Vec<ApiUsage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for usage in batch.drain(..) {
        if let Err(e) = repository.log_usage(usage).await {
            tracing::error!("Failed to log usage: {}", e);
        }
    }
    Ok(())
}

/// Enhanced middleware that uses the background tracker
pub fn create_usage_tracking_middleware(
    tracker: Arc<UsageTracker>,
) -> impl Fn(
    ConnectInfo<SocketAddr>,
    HeaderMap,
    Request,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone {
    move |addr, headers, request, next| {
        let tracker = tracker.clone();
        Box::pin(async move {
            let start_time = Instant::now();
            let method = request.method().to_string();
            let uri = request.uri().path().to_string();
            let user_agent = headers
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            let api_key = headers
                .get("x-api-key")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            let user_id = headers
                .get("x-user-id")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            let request_size = headers
                .get("content-length")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());

            let response = next.run(request).await;

            let response_time = start_time.elapsed().as_millis() as u32;
            let status_code = response.status().as_u16();

            let response_size = response
                .headers()
                .get("content-length")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());

            let usage = ApiUsage::new(
                uri,
                method,
                user_agent,
                addr.0.ip().to_string(),
                status_code,
                response_time,
                request_size,
                response_size,
                api_key,
                user_id,
            );

            tracker.track(usage);
            response
        })
    }
}
