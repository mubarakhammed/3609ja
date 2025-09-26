use uuid::Uuid;

use crate::domain::entities::postal_code::{CreatePostalCodeRequest, UpdatePostalCodeRequest};
use crate::domain::repositories::postal_code_repository::PostalCodeRepository;
use crate::domain::value_objects::{PostalCode as PostalCodeValue, Coordinates};
use crate::application::dtos::{PostalCodeDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// Postal code use cases
#[derive(Clone)]
pub struct PostalCodeUseCases<R: PostalCodeRepository + Clone> {
    postal_code_repository: R,
}

impl<R: PostalCodeRepository + Clone> PostalCodeUseCases<R> {
    pub fn new(postal_code_repository: R) -> Self {
        Self { postal_code_repository }
    }

    /// Get postal codes by ward ID with pagination
    pub async fn get_postal_codes_by_ward(&self, ward_id: Uuid, params: PaginationParams) -> AppResult<PaginatedResponse<PostalCodeDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let postal_codes = self.postal_code_repository.find_by_ward_id(ward_id, page, limit).await?;
        let total = self.postal_code_repository.count_by_ward(ward_id).await?;

        let postal_code_dtos: Vec<PostalCodeDto> = postal_codes.into_iter().map(|p| p.into()).collect();

        Ok(PaginatedResponse::new(postal_code_dtos, page, limit, total))
    }

    /// Get postal codes by ward ID with pagination (alternative method name)
    pub async fn get_postal_codes_by_ward_id(&self, ward_id: Uuid, params: PaginationParams) -> AppResult<PaginatedResponse<PostalCodeDto>> {
        self.get_postal_codes_by_ward(ward_id, params).await
    }

    /// Get postal code by ID
    pub async fn get_postal_code_by_id(&self, id: Uuid) -> AppResult<Option<PostalCodeDto>> {
        let postal_code = self.postal_code_repository.find_by_id(id).await?;
        Ok(postal_code.map(|p| p.into()))
    }

    /// Get postal code by code
    pub async fn get_postal_code_by_code(&self, code: &str) -> AppResult<Option<PostalCodeDto>> {
        let postal_code_value = PostalCodeValue::new(code.to_string())
            .map_err(|e| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
        
        let postal_code = self.postal_code_repository.find_by_code(&postal_code_value).await?;
        Ok(postal_code.map(|p| p.into()))
    }

    /// Find postal codes near coordinates
    pub async fn find_nearby_postal_codes(&self, lat: f64, lng: f64, radius_km: f64) -> AppResult<Vec<PostalCodeDto>> {
        let coordinates = Coordinates::new(lat, lng)
            .map_err(|e| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
        
        let postal_codes = self.postal_code_repository.find_near_coordinates(&coordinates, radius_km).await?;
        Ok(postal_codes.into_iter().map(|p| p.into()).collect())
    }

    /// Find postal codes near coordinates (alternative method name)
    pub async fn find_near_coordinates(&self, coordinates: Coordinates, radius_km: f64) -> AppResult<Vec<PostalCodeDto>> {
        let postal_codes = self.postal_code_repository.find_near_coordinates(&coordinates, radius_km).await?;
        Ok(postal_codes.into_iter().map(|p| p.into()).collect())
    }

    /// Create a new postal code
    pub async fn create_postal_code(&self, ward_id: Uuid, request: CreatePostalCodeRequest) -> AppResult<PostalCodeDto> {
        let postal_code = self.postal_code_repository.create(&request, ward_id).await?;
        Ok(postal_code.into())
    }

    /// Update postal code
    pub async fn update_postal_code(&self, id: Uuid, request: UpdatePostalCodeRequest) -> AppResult<PostalCodeDto> {
        let postal_code = self.postal_code_repository.update(id, &request).await?;
        Ok(postal_code.into())
    }

    /// Delete postal code
    pub async fn delete_postal_code(&self, id: Uuid) -> AppResult<()> {
        self.postal_code_repository.delete(id).await
    }

    /// Search postal codes
    pub async fn search_postal_codes(&self, query: &str, params: PaginationParams) -> AppResult<Vec<PostalCodeDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let postal_codes = self.postal_code_repository.search(query, page, limit).await?;
        Ok(postal_codes.into_iter().map(|p| p.into()).collect())
    }
}