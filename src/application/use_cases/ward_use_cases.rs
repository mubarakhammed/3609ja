use uuid::Uuid;

use crate::domain::entities::ward::{CreateWardRequest, UpdateWardRequest};
use crate::domain::repositories::ward_repository::WardRepository;
use crate::application::dtos::{WardDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// Ward use cases
#[derive(Clone)]
pub struct WardUseCases<R: WardRepository + Clone> {
    ward_repository: R,
}

impl<R: WardRepository + Clone> WardUseCases<R> {
    pub fn new(ward_repository: R) -> Self {
        Self { ward_repository }
    }

    /// Get wards by LGA ID with pagination
    pub async fn get_wards_by_lga(&self, lga_id: Uuid, params: PaginationParams) -> AppResult<PaginatedResponse<WardDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let wards = self.ward_repository.find_by_lga_id(lga_id, page, limit).await?;
        let total = self.ward_repository.count_by_lga(lga_id).await?;

        let ward_dtos: Vec<WardDto> = wards.into_iter().map(|w| w.into()).collect();

        Ok(PaginatedResponse::new(ward_dtos, page, limit, total))
    }

    /// Get ward by ID
    pub async fn get_ward_by_id(&self, id: Uuid) -> AppResult<Option<WardDto>> {
        let ward = self.ward_repository.find_by_id(id).await?;
        Ok(ward.map(|w| w.into()))
    }

    /// Create a new ward
    pub async fn create_ward(&self, lga_id: Uuid, request: CreateWardRequest) -> AppResult<WardDto> {
        let ward = self.ward_repository.create(&request, lga_id).await?;
        Ok(ward.into())
    }

    /// Update ward
    pub async fn update_ward(&self, id: Uuid, request: UpdateWardRequest) -> AppResult<WardDto> {
        let ward = self.ward_repository.update(id, &request).await?;
        Ok(ward.into())
    }

    /// Delete ward
    pub async fn delete_ward(&self, id: Uuid) -> AppResult<()> {
        self.ward_repository.delete(id).await
    }

    /// Search wards
    pub async fn search_wards(&self, query: &str, params: PaginationParams) -> AppResult<Vec<WardDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let wards = self.ward_repository.search(query, page, limit).await?;
        Ok(wards.into_iter().map(|w| w.into()).collect())
    }
}