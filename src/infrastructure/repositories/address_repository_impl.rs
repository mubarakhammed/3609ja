use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::address::{Address, AddressValidationRequest, AddressValidationResponse, AddressSuggestion};
use crate::domain::repositories::address_repository::AddressRepository;
use crate::domain::repositories::{
    state_repository::StateRepository,
    lga_repository::LgaRepository,
    ward_repository::WardRepository,
    postal_code_repository::PostalCodeRepository,
};
use crate::errors::AppResult;

/// PostgreSQL implementation of AddressRepository
pub struct PostgresAddressRepository {
    pool: PgPool,
    state_repo: Box<dyn StateRepository + Send + Sync>,
    lga_repo: Box<dyn LgaRepository + Send + Sync>,
    ward_repo: Box<dyn WardRepository + Send + Sync>,
    postal_code_repo: Box<dyn PostalCodeRepository + Send + Sync>,
}

impl PostgresAddressRepository {
    pub fn new(
        pool: PgPool,
        state_repo: Box<dyn StateRepository + Send + Sync>,
        lga_repo: Box<dyn LgaRepository + Send + Sync>,
        ward_repo: Box<dyn WardRepository + Send + Sync>,
        postal_code_repo: Box<dyn PostalCodeRepository + Send + Sync>,
    ) -> Self {
        Self {
            pool,
            state_repo,
            lga_repo,
            ward_repo,
            postal_code_repo,
        }
    }
}

#[async_trait]
impl AddressRepository for PostgresAddressRepository {
    async fn validate_address(&self, request: &AddressValidationRequest) -> AppResult<AddressValidationResponse> {
        // Find exact matches for each component
        let state = self.state_repo.find_by_name(&request.state).await?;
        let lga = self.lga_repo.find_by_name(&request.lga).await?;
        let ward = self.ward_repo.find_by_name(&request.ward).await?;
        let postal_code_value = crate::domain::value_objects::PostalCode::new(request.postal_code.clone())
            .map_err(|e| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
        let postal_code = self.postal_code_repo.find_by_code(&postal_code_value).await?;

        // Check if all components exist and form a valid address
        let is_valid = state.is_some() && lga.is_some() && ward.is_some() && postal_code.is_some();

        if is_valid {
            // Create canonical address
            let canonical = Address::new(
                state.unwrap(),
                lga.unwrap(),
                ward.unwrap(),
                postal_code.unwrap(),
            );

            return Ok(AddressValidationResponse {
                valid: true,
                canonical: Some(canonical),
                suggestions: vec![],
            });
        }

        // Generate suggestions for invalid addresses
        let suggestions = self.generate_suggestions(request).await?;

        Ok(AddressValidationResponse {
            valid: false,
            canonical: None,
            suggestions,
        })
    }

    async fn find_by_components(
        &self,
        state: &str,
        lga: &str,
        ward: &str,
        postal_code: &str,
    ) -> AppResult<Option<Address>> {
        // Find all components
        let state_entity = self.state_repo.find_by_name(state).await?;
        let lga_entity = self.lga_repo.find_by_name(lga).await?;
        let ward_entity = self.ward_repo.find_by_name(ward).await?;
        let postal_code_value = crate::domain::value_objects::PostalCode::new(postal_code.to_string())
            .map_err(|e| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
        let postal_code_entity = self.postal_code_repo.find_by_code(&postal_code_value).await?;

        match (state_entity, lga_entity, ward_entity, postal_code_entity) {
            (Some(state), Some(lga), Some(ward), Some(postal_code)) => {
                Ok(Some(Address::new(state, lga, ward, postal_code)))
            }
            _ => Ok(None),
        }
    }

    async fn find_similar_addresses(&self, request: &AddressValidationRequest) -> AppResult<Vec<Address>> {
        // Search for similar addresses using fuzzy matching
        let mut similar_addresses = Vec::new();

        // Search for states with similar names
        let states = self.state_repo.search(&request.state, 1, 10).await?;
        for state in states {
            // Search for LGAs in this state
            let lgas = self.lga_repo.search(&request.lga, 1, 10).await?;
            for lga in lgas {
                if lga.state_id == state.id {
                    // Search for wards in this LGA
                    let wards = self.ward_repo.search(&request.ward, 1, 10).await?;
                    for ward in wards {
                        if ward.lga_id == lga.id {
                            // Search for postal codes in this ward
                            let postal_codes = self.postal_code_repo.search(&request.postal_code, 1, 10).await?;
                            for postal_code in postal_codes {
                                if postal_code.ward_id == ward.id {
                                    similar_addresses.push(Address::new(
                                        state.clone(),
                                        lga.clone(),
                                        ward.clone(),
                                        postal_code,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(similar_addresses)
    }
}

impl PostgresAddressRepository {
    async fn generate_suggestions(&self, request: &AddressValidationRequest) -> AppResult<Vec<AddressSuggestion>> {
        let mut suggestions = Vec::new();

        // Find similar states
        let states = self.state_repo.search(&request.state, 1, 5).await?;
        for state in states {
            suggestions.push(AddressSuggestion {
                state: Some(state),
                lga: None,
                ward: None,
                postal_code: None,
                reason: "Similar state name found".to_string(),
                confidence: 0.8,
            });
        }

        // Find similar LGAs
        let lgas = self.lga_repo.search(&request.lga, 1, 5).await?;
        for lga in lgas {
            suggestions.push(AddressSuggestion {
                state: None,
                lga: Some(lga),
                ward: None,
                postal_code: None,
                reason: "Similar LGA name found".to_string(),
                confidence: 0.8,
            });
        }

        // Find similar wards
        let wards = self.ward_repo.search(&request.ward, 1, 5).await?;
        for ward in wards {
            suggestions.push(AddressSuggestion {
                state: None,
                lga: None,
                ward: Some(ward),
                postal_code: None,
                reason: "Similar ward name found".to_string(),
                confidence: 0.8,
            });
        }

        // Find similar postal codes
        let postal_codes = self.postal_code_repo.search(&request.postal_code, 1, 5).await?;
        for postal_code in postal_codes {
            suggestions.push(AddressSuggestion {
                state: None,
                lga: None,
                ward: None,
                postal_code: Some(postal_code),
                reason: "Similar postal code found".to_string(),
                confidence: 0.9,
            });
        }

        Ok(suggestions)
    }
}