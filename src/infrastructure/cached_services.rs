use tracing::{info, warn};
use uuid::Uuid;

use crate::application::dtos::{
    LgaDto, PaginatedResponse, PaginationParams, PostalCodeDto, StateDto, WardDto,
};
use crate::application::use_cases::{
    lga_use_cases::LgaUseCases,
    postal_code_use_cases::PostalCodeUseCases,
    search_use_cases::{SearchResultDto, SearchUseCases},
    state_use_cases::StateUseCases,
    ward_use_cases::WardUseCases,
};
use crate::errors::AppResult;
use crate::infrastructure::{
    cache::{CacheClient, CacheKeys, CacheTTL},
    repositories::{
        lga_repository_impl::PostgresLgaRepository,
        postal_code_repository_impl::PostgresPostalCodeRepository,
        state_repository_impl::PostgresStateRepository,
        ward_repository_impl::PostgresWardRepository,
    },
};

/// Cached service layer that wraps use cases with Redis caching
#[derive(Clone)]
pub struct CachedServices {
    cache: CacheClient,
    state_use_cases: StateUseCases<PostgresStateRepository>,
    lga_use_cases: LgaUseCases<PostgresLgaRepository>,
    ward_use_cases: WardUseCases<PostgresWardRepository>,
    postal_code_use_cases: PostalCodeUseCases<PostgresPostalCodeRepository>,
    search_use_cases: SearchUseCases<
        PostgresStateRepository,
        PostgresLgaRepository,
        PostgresWardRepository,
        PostgresPostalCodeRepository,
    >,
}

impl CachedServices {
    pub fn new(
        cache: CacheClient,
        state_use_cases: StateUseCases<PostgresStateRepository>,
        lga_use_cases: LgaUseCases<PostgresLgaRepository>,
        ward_use_cases: WardUseCases<PostgresWardRepository>,
        postal_code_use_cases: PostalCodeUseCases<PostgresPostalCodeRepository>,
        search_use_cases: SearchUseCases<
            PostgresStateRepository,
            PostgresLgaRepository,
            PostgresWardRepository,
            PostgresPostalCodeRepository,
        >,
    ) -> Self {
        Self {
            cache,
            state_use_cases,
            lga_use_cases,
            ward_use_cases,
            postal_code_use_cases,
            search_use_cases,
        }
    }

    /// Get states with caching
    pub async fn get_states(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<StateDto>> {
        let cache_key = CacheKeys::states();

        // Try to get from cache first
        if let Ok(Some(cached_result)) = self
            .cache
            .get::<PaginatedResponse<StateDto>>(&cache_key)
            .await
        {
            info!("Cache hit for states");
            return Ok(cached_result);
        }

        // Cache miss - get from database
        info!("Cache miss for states - fetching from database");
        let result = self.state_use_cases.get_states(params).await?;

        // Cache the result
        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::STATES).await {
            warn!("Failed to cache states: {}", e);
        }

        Ok(result)
    }

    /// Get state by ID with caching
    pub async fn get_state_by_id(&self, id: Uuid) -> AppResult<Option<StateDto>> {
        let cache_key = CacheKeys::state_by_id(&id);

        if let Ok(Some(cached_result)) = self.cache.get::<Option<StateDto>>(&cache_key).await {
            info!("Cache hit for state {}", id);
            return Ok(cached_result);
        }

        info!("Cache miss for state {} - fetching from database", id);
        let result = self.state_use_cases.get_state_by_id(id).await?;

        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::STATES).await {
            warn!("Failed to cache state {}: {}", id, e);
        }

        Ok(result)
    }

    /// Get LGAs by state with caching
    pub async fn get_lgas_by_state(
        &self,
        state_id: Uuid,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<LgaDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);
        let cache_key = CacheKeys::lgas_by_state(&state_id, page, limit);

        if let Ok(Some(cached_result)) = self
            .cache
            .get::<PaginatedResponse<LgaDto>>(&cache_key)
            .await
        {
            info!("Cache hit for LGAs in state {}", state_id);
            return Ok(cached_result);
        }

        info!(
            "Cache miss for LGAs in state {} - fetching from database",
            state_id
        );
        let result = self
            .lga_use_cases
            .get_lgas_by_state(state_id, params)
            .await?;

        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::LGAS).await {
            warn!("Failed to cache LGAs for state {}: {}", state_id, e);
        }

        Ok(result)
    }

    /// Get LGA by ID with caching
    pub async fn get_lga_by_id(&self, id: Uuid) -> AppResult<Option<LgaDto>> {
        let cache_key = CacheKeys::lga_by_id(&id);

        if let Ok(Some(cached_result)) = self.cache.get::<Option<LgaDto>>(&cache_key).await {
            info!("Cache hit for LGA {}", id);
            return Ok(cached_result);
        }

        info!("Cache miss for LGA {} - fetching from database", id);
        let result = self.lga_use_cases.get_lga_by_id(id).await?;

        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::LGAS).await {
            warn!("Failed to cache LGA {}: {}", id, e);
        }

        Ok(result)
    }

    /// Get wards by LGA with caching
    pub async fn get_wards_by_lga(
        &self,
        lga_id: Uuid,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<WardDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);
        let cache_key = CacheKeys::wards_by_lga(&lga_id, page, limit);

        if let Ok(Some(cached_result)) = self
            .cache
            .get::<PaginatedResponse<WardDto>>(&cache_key)
            .await
        {
            info!("Cache hit for wards in LGA {}", lga_id);
            return Ok(cached_result);
        }

        info!(
            "Cache miss for wards in LGA {} - fetching from database",
            lga_id
        );
        let result = self.ward_use_cases.get_wards_by_lga(lga_id, params).await?;

        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::WARDS).await {
            warn!("Failed to cache wards for LGA {}: {}", lga_id, e);
        }

        Ok(result)
    }

    /// Get ward by ID with caching
    pub async fn get_ward_by_id(&self, id: Uuid) -> AppResult<Option<WardDto>> {
        let cache_key = CacheKeys::ward_by_id(&id);

        if let Ok(Some(cached_result)) = self.cache.get::<Option<WardDto>>(&cache_key).await {
            info!("Cache hit for ward {}", id);
            return Ok(cached_result);
        }

        info!("Cache miss for ward {} - fetching from database", id);
        let result = self.ward_use_cases.get_ward_by_id(id).await?;

        if let Err(e) = self.cache.set(&cache_key, &result, CacheTTL::WARDS).await {
            warn!("Failed to cache ward {}: {}", id, e);
        }

        Ok(result)
    }

    /// Get postal codes by ward with caching
    pub async fn get_postal_codes_by_ward(
        &self,
        ward_id: Uuid,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<PostalCodeDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);
        let cache_key = CacheKeys::postal_codes_by_ward(&ward_id, page, limit);

        if let Ok(Some(cached_result)) = self
            .cache
            .get::<PaginatedResponse<PostalCodeDto>>(&cache_key)
            .await
        {
            info!("Cache hit for postal codes in ward {}", ward_id);
            return Ok(cached_result);
        }

        info!(
            "Cache miss for postal codes in ward {} - fetching from database",
            ward_id
        );
        let result = self
            .postal_code_use_cases
            .get_postal_codes_by_ward(ward_id, params)
            .await?;

        if let Err(e) = self
            .cache
            .set(&cache_key, &result, CacheTTL::POSTAL_CODES)
            .await
        {
            warn!("Failed to cache postal codes for ward {}: {}", ward_id, e);
        }

        Ok(result)
    }

    /// Get postal code by ID with caching
    pub async fn get_postal_code_by_id(&self, id: Uuid) -> AppResult<Option<PostalCodeDto>> {
        let cache_key = CacheKeys::postal_code_by_id(&id);

        if let Ok(Some(cached_result)) = self.cache.get::<Option<PostalCodeDto>>(&cache_key).await {
            info!("Cache hit for postal code {}", id);
            return Ok(cached_result);
        }

        info!("Cache miss for postal code {} - fetching from database", id);
        let result = self.postal_code_use_cases.get_postal_code_by_id(id).await?;

        if let Err(e) = self
            .cache
            .set(&cache_key, &result, CacheTTL::POSTAL_CODES)
            .await
        {
            warn!("Failed to cache postal code {}: {}", id, e);
        }

        Ok(result)
    }

    /// Get postal code by code with caching
    pub async fn get_postal_code_by_code(&self, code: &str) -> AppResult<Option<PostalCodeDto>> {
        let cache_key = CacheKeys::postal_code_by_code(code);

        if let Ok(Some(cached_result)) = self.cache.get::<Option<PostalCodeDto>>(&cache_key).await {
            info!("Cache hit for postal code {}", code);
            return Ok(cached_result);
        }

        info!(
            "Cache miss for postal code {} - fetching from database",
            code
        );
        let result = self
            .postal_code_use_cases
            .get_postal_code_by_code(code)
            .await?;

        if let Err(e) = self
            .cache
            .set(&cache_key, &result, CacheTTL::POSTAL_CODES)
            .await
        {
            warn!("Failed to cache postal code {}: {}", code, e);
        }

        Ok(result)
    }

    /// Search all with caching
    pub async fn search_all(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> AppResult<SearchResultDto> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);
        let cache_key = CacheKeys::search_results(query, page, limit);

        if let Ok(Some(cached_result)) = self.cache.get::<SearchResultDto>(&cache_key).await {
            info!("Cache hit for search query: {}", query);
            return Ok(cached_result);
        }

        info!(
            "Cache miss for search query: {} - fetching from database",
            query
        );
        let result = self.search_use_cases.search_all(query, params).await?;

        if let Err(e) = self
            .cache
            .set(&cache_key, &result, CacheTTL::SEARCH_RESULTS)
            .await
        {
            warn!("Failed to cache search results for query {}: {}", query, e);
        }

        Ok(result)
    }

    // Postal code nearby search (geographic queries shouldn't be cached as aggressively)
    pub async fn find_nearby_postal_codes(
        &self,
        lat: f64,
        lng: f64,
        radius_km: f64,
    ) -> AppResult<Vec<PostalCodeDto>> {
        // Geographic queries are dynamic and less suitable for caching
        // Could implement short-term caching with location-based keys if needed
        self.postal_code_use_cases
            .find_nearby_postal_codes(lat, lng, radius_km)
            .await
    }

    // Individual search methods that delegate to search_all and extract relevant parts
    pub async fn search_states(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> AppResult<Vec<StateDto>> {
        let search_result = self.search_all(query, params).await?;
        Ok(search_result.states)
    }

    pub async fn search_lgas(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> AppResult<Vec<LgaDto>> {
        let search_result = self.search_all(query, params).await?;
        Ok(search_result.lgas)
    }

    pub async fn search_wards(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> AppResult<Vec<WardDto>> {
        let search_result = self.search_all(query, params).await?;
        Ok(search_result.wards)
    }

    pub async fn search_postal_codes(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> AppResult<Vec<PostalCodeDto>> {
        let search_result = self.search_all(query, params).await?;
        Ok(search_result.postal_codes)
    }

    /// Access to underlying use cases for methods that shouldn't be cached
    pub fn state_use_cases(&self) -> &StateUseCases<PostgresStateRepository> {
        &self.state_use_cases
    }

    pub fn lga_use_cases(&self) -> &LgaUseCases<PostgresLgaRepository> {
        &self.lga_use_cases
    }

    pub fn ward_use_cases(&self) -> &WardUseCases<PostgresWardRepository> {
        &self.ward_use_cases
    }

    pub fn postal_code_use_cases(&self) -> &PostalCodeUseCases<PostgresPostalCodeRepository> {
        &self.postal_code_use_cases
    }

    pub fn search_use_cases(
        &self,
    ) -> &SearchUseCases<
        PostgresStateRepository,
        PostgresLgaRepository,
        PostgresWardRepository,
        PostgresPostalCodeRepository,
    > {
        &self.search_use_cases
    }

    /// Access to the cache client for health checks and direct operations
    pub fn cache_client(&self) -> &CacheClient {
        &self.cache
    }
}
