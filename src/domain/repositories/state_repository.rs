use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::state::{State, CreateStateRequest, UpdateStateRequest};
use crate::domain::value_objects::StateCode;
use crate::errors::AppResult;

/// State repository interface
#[async_trait]
pub trait StateRepository: Send + Sync {
    /// Create a new state
    async fn create(&self, request: &CreateStateRequest) -> AppResult<State>;
    
    /// Find state by ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<State>>;
    
    /// Find state by name
    async fn find_by_name(&self, name: &str) -> AppResult<Option<State>>;
    
    /// Find state by code
    async fn find_by_code(&self, code: &StateCode) -> AppResult<Option<State>>;
    
    /// Update state
    async fn update(&self, id: Uuid, request: &UpdateStateRequest) -> AppResult<State>;
    
    /// Delete state
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// List all states with pagination
    async fn list(&self, page: u32, limit: u32) -> AppResult<Vec<State>>;
    
    /// Count total states
    async fn count(&self) -> AppResult<u64>;
    
    /// Search states by name or code
    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<State>>;
}
