use crate::domain::repositories::{
    state_repository::StateRepository,
    lga_repository::LgaRepository,
    ward_repository::WardRepository,
    postal_code_repository::PostalCodeRepository,
};
use crate::application::dtos::{StateDto, LgaDto, WardDto, PostalCodeDto, PaginationParams};
use crate::errors::AppResult;

/// Search result DTO
#[derive(Debug, serde::Serialize)]
pub struct SearchResultDto {
    pub states: Vec<StateDto>,
    pub lgas: Vec<LgaDto>,
    pub wards: Vec<WardDto>,
    pub postal_codes: Vec<PostalCodeDto>,
}

/// Search use cases
#[derive(Clone)]
pub struct SearchUseCases<
    SR: StateRepository + Clone,
    LR: LgaRepository + Clone,
    WR: WardRepository + Clone,
    PR: PostalCodeRepository + Clone,
> {
    state_repository: SR,
    lga_repository: LR,
    ward_repository: WR,
    postal_code_repository: PR,
}

impl<SR: StateRepository + Clone, LR: LgaRepository + Clone, WR: WardRepository + Clone, PR: PostalCodeRepository + Clone>
    SearchUseCases<SR, LR, WR, PR>
{
    pub fn new(
        state_repository: SR,
        lga_repository: LR,
        ward_repository: WR,
        postal_code_repository: PR,
    ) -> Self {
        Self {
            state_repository,
            lga_repository,
            ward_repository,
            postal_code_repository,
        }
    }

    /// Search across all entities
    pub async fn search_all(&self, query: &str, params: PaginationParams) -> AppResult<SearchResultDto> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        // Search all entities in parallel
        let (states, lgas, wards, postal_codes) = tokio::try_join!(
            self.state_repository.search(query, page, limit),
            self.lga_repository.search(query, page, limit),
            self.ward_repository.search(query, page, limit),
            self.postal_code_repository.search(query, page, limit)
        )?;

        Ok(SearchResultDto {
            states: states.into_iter().map(|s| s.into()).collect(),
            lgas: lgas.into_iter().map(|l| l.into()).collect(),
            wards: wards.into_iter().map(|w| w.into()).collect(),
            postal_codes: postal_codes.into_iter().map(|p| p.into()).collect(),
        })
    }

    /// Search states only
    pub async fn search_states(&self, query: &str, params: PaginationParams) -> AppResult<Vec<StateDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let states = self.state_repository.search(query, page, limit).await?;
        Ok(states.into_iter().map(|s| s.into()).collect())
    }

    /// Search LGAs only
    pub async fn search_lgas(&self, query: &str, params: PaginationParams) -> AppResult<Vec<LgaDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let lgas = self.lga_repository.search(query, page, limit).await?;
        Ok(lgas.into_iter().map(|l| l.into()).collect())
    }

    /// Search wards only
    pub async fn search_wards(&self, query: &str, params: PaginationParams) -> AppResult<Vec<WardDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let wards = self.ward_repository.search(query, page, limit).await?;
        Ok(wards.into_iter().map(|w| w.into()).collect())
    }

    /// Search postal codes only
    pub async fn search_postal_codes(&self, query: &str, params: PaginationParams) -> AppResult<Vec<PostalCodeDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let postal_codes = self.postal_code_repository.search(query, page, limit).await?;
        Ok(postal_codes.into_iter().map(|p| p.into()).collect())
    }
}
