use crate::infrastructure::repositories::{
    state_repository_impl::PostgresStateRepository,
    lga_repository_impl::PostgresLgaRepository,
    ward_repository_impl::PostgresWardRepository,
    postal_code_repository_impl::PostgresPostalCodeRepository,
    address_repository_impl::PostgresAddressRepository,
};

use crate::application::use_cases::{
    state_use_cases::StateUseCases,
    lga_use_cases::LgaUseCases,
    ward_use_cases::WardUseCases,
    postal_code_use_cases::PostalCodeUseCases,
    address_use_cases::AddressUseCases,
    search_use_cases::SearchUseCases,
};

use crate::presentation::controllers::{
    state_controller::StateController,
    lga_controller::LgaController,
    ward_controller::WardController,
    postal_code_controller::PostalCodeController,
    address_controller::AddressController,
    search_controller::SearchController,
};

/// Unified application state containing all controllers
#[derive(Clone)]
pub struct AppState {
    pub state_controller: StateController<PostgresStateRepository>,
    pub lga_controller: LgaController<PostgresLgaRepository>,
    pub ward_controller: WardController<PostgresWardRepository>,
    pub postal_code_controller: PostalCodeController<PostgresPostalCodeRepository>,
    pub address_controller: AddressController<PostgresAddressRepository>,
    pub search_controller: SearchController<PostgresStateRepository, PostgresLgaRepository, PostgresWardRepository, PostgresPostalCodeRepository>,
}

impl AppState {
    pub fn new(
        state_repository: PostgresStateRepository,
        lga_repository: PostgresLgaRepository,
        ward_repository: PostgresWardRepository,
        postal_code_repository: PostgresPostalCodeRepository,
        address_repository: PostgresAddressRepository,
    ) -> Self {
        // Initialize use cases
        let state_use_cases = StateUseCases::new(state_repository.clone());
        let lga_use_cases = LgaUseCases::new(lga_repository.clone());
        let ward_use_cases = WardUseCases::new(ward_repository.clone());
        let postal_code_use_cases = PostalCodeUseCases::new(postal_code_repository.clone());
        let address_use_cases = AddressUseCases::new(address_repository);
        let search_use_cases = SearchUseCases::new(
            state_repository.clone(),
            lga_repository.clone(),
            ward_repository.clone(),
            postal_code_repository.clone(),
        );

        // Initialize controllers
        let state_controller = StateController::new(state_use_cases);
        let lga_controller = LgaController::new(lga_use_cases);
        let ward_controller = WardController::new(ward_use_cases);
        let postal_code_controller = PostalCodeController::new(postal_code_use_cases);
        let address_controller = AddressController::new(address_use_cases);
        let search_controller = SearchController::new(search_use_cases);

        Self {
            state_controller,
            lga_controller,
            ward_controller,
            postal_code_controller,
            address_controller,
            search_controller,
        }
    }
}
