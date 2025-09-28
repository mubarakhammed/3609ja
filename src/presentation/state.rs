use crate::infrastructure::{
    cache::CacheClient,
    cached_services::CachedServices,
    repositories::{
        address_repository_impl::PostgresAddressRepository,
        api_usage_repository_impl::PostgresApiUsageRepository,
        lga_repository_impl::PostgresLgaRepository,
        postal_code_repository_impl::PostgresPostalCodeRepository,
        state_repository_impl::PostgresStateRepository,
        ward_repository_impl::PostgresWardRepository,
    },
};

use crate::application::use_cases::address_use_cases::AddressUseCases;
use sqlx::PgPool;
use std::sync::Arc;

/// Unified application state with caching support
#[derive(Clone)]
pub struct AppState {
    /// Cached services for frequently accessed data
    pub cached_services: CachedServices,
    /// Non-cached services for operations that shouldn't be cached
    pub address_use_cases: Arc<AddressUseCases<PostgresAddressRepository>>,
    /// API usage tracking repository
    pub api_usage_repository: Arc<PostgresApiUsageRepository>,
    /// Database connection pool for health checks and direct access
    pub pool: PgPool,
}

impl AppState {
    pub fn new(
        cache: CacheClient,
        state_repository: PostgresStateRepository,
        lga_repository: PostgresLgaRepository,
        ward_repository: PostgresWardRepository,
        postal_code_repository: PostgresPostalCodeRepository,
        address_repository: PostgresAddressRepository,
        api_usage_repository: PostgresApiUsageRepository,
        pool: PgPool,
    ) -> Self {
        use crate::application::use_cases::{
            lga_use_cases::LgaUseCases, postal_code_use_cases::PostalCodeUseCases,
            search_use_cases::SearchUseCases, state_use_cases::StateUseCases,
            ward_use_cases::WardUseCases,
        };

        // Initialize use cases
        let state_use_cases = StateUseCases::new(state_repository.clone());
        let lga_use_cases = LgaUseCases::new(lga_repository.clone());
        let ward_use_cases = WardUseCases::new(ward_repository.clone());
        let postal_code_use_cases = PostalCodeUseCases::new(postal_code_repository.clone());
        let search_use_cases = SearchUseCases::new(
            state_repository,
            lga_repository,
            ward_repository,
            postal_code_repository,
        );

        // Initialize cached services
        let cached_services = CachedServices::new(
            cache,
            state_use_cases,
            lga_use_cases,
            ward_use_cases,
            postal_code_use_cases,
            search_use_cases,
        );

        // Address use cases don't need caching (they're for validation/complex operations)
        let address_use_cases = Arc::new(AddressUseCases::new(address_repository));
        let api_usage_repository = Arc::new(api_usage_repository);

        Self {
            cached_services,
            address_use_cases,
            api_usage_repository,
            pool,
        }
    }

    /// Access to the database pool for health checks
    pub fn database_pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}
