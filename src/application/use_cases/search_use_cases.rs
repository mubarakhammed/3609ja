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
pub struct SearchUseCases<
    SR: StateRepository,
    LR: LgaRepository,
    WR: WardRepository,
    PR: PostalCodeRepository,
> {
    state_repository: SR,
    lga_repository: LR,
    ward_repository: WR,
    postal_code_repository: PR,
}

impl<SR: StateRepository, LR: LgaRepository, WR: WardRepository, PR: PostalCodeRepository>
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
}
