use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::state::{State, CreateStateRequest, UpdateStateRequest};
use crate::domain::repositories::state_repository::StateRepository;
use crate::domain::value_objects::{StateCode, StateCodeError};
use crate::errors::AppResult;

/// PostgreSQL implementation of StateRepository
#[derive(Clone)]
pub struct PostgresStateRepository {
    pool: PgPool,
}

impl PostgresStateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StateRepository for PostgresStateRepository {
    async fn create(&self, request: &CreateStateRequest) -> AppResult<State> {
        let state_code = StateCode::new(request.code.clone())
            .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

        let state = State::new(request.name.clone(), state_code);

        sqlx::query(
            "INSERT INTO states (id, name, code, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&state.id)
        .bind(&state.name)
        .bind(state.code.to_string())
        .bind(&state.created_at)
        .bind(&state.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(state)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<State>> {
        let row = sqlx::query(
            "SELECT id, name, code, created_at, updated_at FROM states WHERE id = $1"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let state_code = StateCode::new(row.get::<String, _>("code"))
                    .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(State {
                    id: row.get("id"),
                    name: row.get("name"),
                    code: state_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<State>> {
        let row = sqlx::query(
            "SELECT id, name, code, created_at, updated_at FROM states WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let state_code = StateCode::new(row.get::<String, _>("code"))
                    .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(State {
                    id: row.get("id"),
                    name: row.get("name"),
                    code: state_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_code(&self, code: &StateCode) -> AppResult<Option<State>> {
        let row = sqlx::query(
            "SELECT id, name, code, created_at, updated_at FROM states WHERE code = $1"
        )
        .bind(code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let state_code = StateCode::new(row.get::<String, _>("code"))
                    .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(State {
                    id: row.get("id"),
                    name: row.get("name"),
                    code: state_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, id: Uuid, request: &UpdateStateRequest) -> AppResult<State> {
        let mut state = self.find_by_id(id).await?
            .ok_or_else(|| crate::errors::AppError::NotFound("State not found".to_string()))?;

        if let Some(name) = &request.name {
            state.update_name(name.clone());
        }

        if let Some(code) = &request.code {
            let state_code = StateCode::new(code.clone())
                .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            state.update_code(state_code);
        }

        sqlx::query(
            "UPDATE states SET name = $1, code = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(&state.name)
        .bind(state.code.to_string())
        .bind(&state.updated_at)
        .bind(&state.id)
        .execute(&self.pool)
        .await?;

        Ok(state)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM states WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound("State not found".to_string()));
        }

        Ok(())
    }

    async fn list(&self, page: u32, limit: u32) -> AppResult<Vec<State>> {
        let offset = (page - 1) * limit;
        
        let rows = sqlx::query(
            "SELECT id, name, code, created_at, updated_at FROM states ORDER BY name LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut states = Vec::new();
        for row in rows {
            let state_code = StateCode::new(row.get::<String, _>("code"))
                .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            states.push(State {
                id: row.get("id"),
                name: row.get("name"),
                code: state_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(states)
    }

    async fn count(&self) -> AppResult<u64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM states")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count") as u64)
    }

    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<State>> {
        let offset = (page - 1) * limit;
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            "SELECT id, name, code, created_at, updated_at FROM states WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(&search_pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut states = Vec::new();
        for row in rows {
            let state_code = StateCode::new(row.get::<String, _>("code"))
                .map_err(|e: StateCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            states.push(State {
                id: row.get("id"),
                name: row.get("name"),
                code: state_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(states)
    }
}