use redis::{Client, Connection, RedisResult};
use serde::{Deserialize, Serialize};

use tracing::{debug, error, info};

/// Redis cache client wrapper
#[derive(Clone)]
pub struct CacheClient {
    client: Client,
}

impl CacheClient {
    /// Create a new cache client
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        info!("Redis client created successfully");
        Ok(Self { client })
    }

    /// Get connection to Redis
    async fn get_connection(&self) -> RedisResult<Connection> {
        self.client.get_connection()
    }

    /// Set a value in cache with expiration
    pub async fn set<T>(&self, key: &str, value: &T, ttl_seconds: u64) -> RedisResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization failed",
                e.to_string(),
            ))
        })?;

        let mut con = self.get_connection().await?;
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_seconds)
            .arg(serialized)
            .query(&mut con)?;

        debug!("Cached key: {} with TTL: {}s", key, ttl_seconds);
        Ok(())
    }

    /// Get a value from cache
    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut con = self.get_connection().await?;
        let result: Option<String> = redis::cmd("GET").arg(key).query(&mut con)?;

        match result {
            Some(serialized) => match serde_json::from_str(&serialized) {
                Ok(value) => {
                    debug!("Cache hit for key: {}", key);
                    Ok(Some(value))
                }
                Err(e) => {
                    error!("Failed to deserialize cached value for key {}: {}", key, e);
                    Ok(None)
                }
            },
            None => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }

    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut con = self.get_connection().await?;
        redis::cmd("DEL").arg(key).query(&mut con)?;

        debug!("Deleted cache key: {}", key);
        Ok(())
    }

    /// Check if key exists in cache
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut con = self.get_connection().await?;
        let exists: bool = redis::cmd("EXISTS").arg(key).query(&mut con)?;
        Ok(exists)
    }

    /// Set cache with no expiration
    pub async fn set_permanent<T>(&self, key: &str, value: &T) -> RedisResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization failed",
                e.to_string(),
            ))
        })?;

        let mut con = self.get_connection().await?;
        redis::cmd("SET").arg(key).arg(serialized).query(&mut con)?;

        debug!("Permanently cached key: {}", key);
        Ok(())
    }

    /// Increment a counter (for rate limiting)
    pub async fn increment(&self, key: &str, ttl_seconds: u64) -> RedisResult<i64> {
        let mut con = self.get_connection().await?;

        // Increment the key
        let count: i64 = redis::cmd("INCR").arg(key).query(&mut con)?;

        // Set expiration if this is the first increment
        if count == 1 {
            redis::cmd("EXPIRE")
                .arg(key)
                .arg(ttl_seconds)
                .query(&mut con)?;
        }

        Ok(count)
    }
}

/// Cache key builders
pub struct CacheKeys;

impl CacheKeys {
    pub fn states() -> String {
        "nigeria_geo:states:all".to_string()
    }

    pub fn state_by_id(id: &uuid::Uuid) -> String {
        format!("nigeria_geo:state:{}", id)
    }

    pub fn lgas_by_state(state_id: &uuid::Uuid, page: u32, limit: u32) -> String {
        format!("nigeria_geo:state:{}:lgas:p{}:l{}", state_id, page, limit)
    }

    pub fn lga_by_id(id: &uuid::Uuid) -> String {
        format!("nigeria_geo:lga:{}", id)
    }

    pub fn wards_by_lga(lga_id: &uuid::Uuid, page: u32, limit: u32) -> String {
        format!("nigeria_geo:lga:{}:wards:p{}:l{}", lga_id, page, limit)
    }

    pub fn ward_by_id(id: &uuid::Uuid) -> String {
        format!("nigeria_geo:ward:{}", id)
    }

    pub fn postal_codes_by_ward(ward_id: &uuid::Uuid, page: u32, limit: u32) -> String {
        format!(
            "nigeria_geo:ward:{}:postal_codes:p{}:l{}",
            ward_id, page, limit
        )
    }

    pub fn postal_code_by_id(id: &uuid::Uuid) -> String {
        format!("nigeria_geo:postal_code:{}", id)
    }

    pub fn postal_code_by_code(code: &str) -> String {
        format!("nigeria_geo:postal_code:code:{}", code)
    }

    pub fn search_results(query: &str, page: u32, limit: u32) -> String {
        format!("nigeria_geo:search:{}:p{}:l{}", query, page, limit)
    }

    pub fn rate_limit(identifier: &str) -> String {
        format!("nigeria_geo:rate_limit:{}", identifier)
    }
}

/// Cache TTL constants (in seconds)
pub struct CacheTTL;

impl CacheTTL {
    pub const STATES: u64 = 3600; // 1 hour (states don't change often)
    pub const LGAS: u64 = 1800; // 30 minutes
    pub const WARDS: u64 = 1800; // 30 minutes
    pub const POSTAL_CODES: u64 = 900; // 15 minutes
    pub const SEARCH_RESULTS: u64 = 600; // 10 minutes
    pub const RATE_LIMIT_WINDOW: u64 = 60; // 1 minute
}
