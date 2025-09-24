use uuid::Uuid;

use crate::domain::entities::state::{CreateStateRequest, UpdateStateRequest};
use crate::domain::repositories::state_repository::StateRepository;
use crate::application::dtos::{StateDto, PaginatedResponse, PaginationParams};
use crate::errors::AppResult;

/// State use cases
#[derive(Clone)]
pub struct StateUseCases<R: StateRepository + Clone> {
    state_repository: R,
}

impl<R: StateRepository + Clone> StateUseCases<R> {
    pub fn new(state_repository: R) -> Self {
        Self { state_repository }
    }

    /// Get all states with pagination
    pub async fn get_states(&self, params: PaginationParams) -> AppResult<PaginatedResponse<StateDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let states = self.state_repository.list(page, limit).await?;
        let total = self.state_repository.count().await?;

        let state_dtos: Vec<StateDto> = states.into_iter().map(|s| s.into()).collect();

        Ok(PaginatedResponse::new(state_dtos, page, limit, total))
    }

    /// Get state by ID
    pub async fn get_state_by_id(&self, id: Uuid) -> AppResult<Option<StateDto>> {
        let state = self.state_repository.find_by_id(id).await?;
        Ok(state.map(|s| s.into()))
    }

    /// Create a new state
    pub async fn create_state(&self, request: CreateStateRequest) -> AppResult<StateDto> {
        let state = self.state_repository.create(&request).await?;
        Ok(state.into())
    }

    /// Update state
    pub async fn update_state(&self, id: Uuid, request: UpdateStateRequest) -> AppResult<StateDto> {
        let state = self.state_repository.update(id, &request).await?;
        Ok(state.into())
    }

    /// Delete state
    pub async fn delete_state(&self, id: Uuid) -> AppResult<()> {
        self.state_repository.delete(id).await
    }

    /// Search states
    pub async fn search_states(&self, query: &str, params: PaginationParams) -> AppResult<Vec<StateDto>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);

        let states = self.state_repository.search(query, page, limit).await?;
        Ok(states.into_iter().map(|s| s.into()).collect())
    }
}
