use uuid::Uuid;

use crate::domain::entities::lga::{CreateLgaRequest, UpdateLgaRequest};
use crate::domain::repositories::lga_repository::LgaRepository;
use crate::application::dtos::{LgaDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// LGA use cases
#[derive(Clone)]
pub struct LgaUseCases<R: LgaRepository + Clone> {
    lga_repository: R,
}

impl<R: LgaRepository + Clone> LgaUseCases<R> {
    pub fn new(lga_repository: R) -> Self {
        Self { lga_repository }
    }

    /// Get LGAs by state ID with pagination
    pub async fn get_lgas_by_state(&self, state_id: Uuid, params: PaginationParams) -> AppResult<PaginatedResponse<LgaDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let lgas = self.lga_repository.find_by_state_id(state_id, page, limit).await?;
        let total = self.lga_repository.count_by_state(state_id).await?;

        let lga_dtos: Vec<LgaDto> = lgas.into_iter().map(|l| l.into()).collect();

        Ok(PaginatedResponse::new(lga_dtos, page, limit, total))
    }

    /// Get LGAs by state ID with pagination (alternative method name)
    pub async fn get_lgas_by_state_id(&self, state_id: Uuid, params: PaginationParams) -> AppResult<PaginatedResponse<LgaDto>> {
        self.get_lgas_by_state(state_id, params).await
    }

    /// Get LGA by ID
    pub async fn get_lga_by_id(&self, id: Uuid) -> AppResult<Option<LgaDto>> {
        let lga = self.lga_repository.find_by_id(id).await?;
        Ok(lga.map(|l| l.into()))
    }

    /// Create a new LGA
    pub async fn create_lga(&self, state_id: Uuid, request: CreateLgaRequest) -> AppResult<LgaDto> {
        let lga = self.lga_repository.create(&request, state_id).await?;
        Ok(lga.into())
    }

    /// Update LGA
    pub async fn update_lga(&self, id: Uuid, request: UpdateLgaRequest) -> AppResult<LgaDto> {
        let lga = self.lga_repository.update(id, &request).await?;
        Ok(lga.into())
    }

    /// Delete LGA
    pub async fn delete_lga(&self, id: Uuid) -> AppResult<()> {
        self.lga_repository.delete(id).await
    }

    /// Search LGAs
    pub async fn search_lgas(&self, query: &str, params: PaginationParams) -> AppResult<Vec<LgaDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let lgas = self.lga_repository.search(query, page, limit).await?;
        Ok(lgas.into_iter().map(|l| l.into()).collect())
    }
}