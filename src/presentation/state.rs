use crate::infrastructure::repositories::{
    address_repository_impl::PostgresAddressRepository,
    api_usage_repository_impl::PostgresApiUsageRepository,
    lga_repository_impl::PostgresLgaRepository,
    postal_code_repository_impl::PostgresPostalCodeRepository,
    state_repository_impl::PostgresStateRepository, ward_repository_impl::PostgresWardRepository,
};

use crate::application::use_cases::{
    address_use_cases::AddressUseCases, lga_use_cases::LgaUseCases,
    postal_code_use_cases::PostalCodeUseCases, search_use_cases::SearchUseCases,
    state_use_cases::StateUseCases, ward_use_cases::WardUseCases,
};
use sqlx::PgPool;
use std::sync::Arc;

/// Simplified application state without caching
#[derive(Clone)]
pub struct AppState {
    /// Direct use cases without caching
    pub state_use_cases: Arc<StateUseCases<PostgresStateRepository>>,
    pub lga_use_cases: Arc<LgaUseCases<PostgresLgaRepository>>,
    pub ward_use_cases: Arc<WardUseCases<PostgresWardRepository>>,
    pub postal_code_use_cases: Arc<PostalCodeUseCases<PostgresPostalCodeRepository>>,
    pub search_use_cases: Arc<
        SearchUseCases<
            PostgresStateRepository,
            PostgresLgaRepository,
            PostgresWardRepository,
            PostgresPostalCodeRepository,
        >,
    >,
    pub address_use_cases: Arc<AddressUseCases<PostgresAddressRepository>>,
    /// API usage tracking repository
    pub api_usage_repository: Arc<PostgresApiUsageRepository>,
    /// Database connection pool for health checks and direct access
    pub pool: PgPool,
}

impl AppState {
    pub fn new(
        state_repository: PostgresStateRepository,
        lga_repository: PostgresLgaRepository,
        ward_repository: PostgresWardRepository,
        postal_code_repository: PostgresPostalCodeRepository,
        address_repository: PostgresAddressRepository,
        api_usage_repository: PostgresApiUsageRepository,
        pool: PgPool,
    ) -> Self {
        // Initialize use cases directly without caching
        let state_use_cases = Arc::new(StateUseCases::new(state_repository.clone()));
        let lga_use_cases = Arc::new(LgaUseCases::new(lga_repository.clone()));
        let ward_use_cases = Arc::new(WardUseCases::new(ward_repository.clone()));
        let postal_code_use_cases =
            Arc::new(PostalCodeUseCases::new(postal_code_repository.clone()));
        let search_use_cases = Arc::new(SearchUseCases::new(
            state_repository,
            lga_repository,
            ward_repository,
            postal_code_repository,
        ));

        let address_use_cases = Arc::new(AddressUseCases::new(address_repository));
        let api_usage_repository = Arc::new(api_usage_repository);

        Self {
            state_use_cases,
            lga_use_cases,
            ward_use_cases,
            postal_code_use_cases,
            search_use_cases,
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
