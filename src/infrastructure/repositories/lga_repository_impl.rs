use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::lga::{Lga, CreateLgaRequest, UpdateLgaRequest};
use crate::domain::repositories::lga_repository::LgaRepository;
use crate::domain::value_objects::{LgaCode, LgaCodeError};
use crate::errors::AppResult;

/// PostgreSQL implementation of LgaRepository
#[derive(Clone)]
pub struct PostgresLgaRepository {
    pool: PgPool,
}

impl PostgresLgaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LgaRepository for PostgresLgaRepository {
    async fn create(&self, request: &CreateLgaRequest, state_id: Uuid) -> AppResult<Lga> {
        let lga_code = LgaCode::new(request.code.clone())
            .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

        let lga = Lga::new(state_id, request.name.clone(), lga_code);

        sqlx::query(
            "INSERT INTO lgas (id, state_id, name, code, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&lga.id)
        .bind(&lga.state_id)
        .bind(&lga.name)
        .bind(lga.code.to_string())
        .bind(&lga.created_at)
        .bind(&lga.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(lga)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Lga>> {
        let row = sqlx::query(
            "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE id = $1"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let lga_code = LgaCode::new(row.get::<String, _>("code"))
                    .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Lga {
                    id: row.get("id"),
                    state_id: row.get("state_id"),
                    name: row.get("name"),
                    code: lga_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Lga>> {
        let row = sqlx::query(
            "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let lga_code = LgaCode::new(row.get::<String, _>("code"))
                    .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Lga {
                    id: row.get("id"),
                    state_id: row.get("state_id"),
                    name: row.get("name"),
                    code: lga_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_code(&self, code: &LgaCode) -> AppResult<Option<Lga>> {
        let row = sqlx::query(
            "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE code = $1"
        )
        .bind(code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let lga_code = LgaCode::new(row.get::<String, _>("code"))
                    .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
                
                Ok(Some(Lga {
                    id: row.get("id"),
                    state_id: row.get("state_id"),
                    name: row.get("name"),
                    code: lga_code,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_state_id(&self, state_id: Uuid, page: u32, limit: u32) -> AppResult<Vec<Lga>> {
        let offset = (page - 1) * limit;
        
        let rows = sqlx::query(
            "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE state_id = $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(&state_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut lgas = Vec::new();
        for row in rows {
            let lga_code = LgaCode::new(row.get::<String, _>("code"))
                .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            lgas.push(Lga {
                id: row.get("id"),
                state_id: row.get("state_id"),
                name: row.get("name"),
                code: lga_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(lgas)
    }

    async fn update(&self, id: Uuid, request: &UpdateLgaRequest) -> AppResult<Lga> {
        let mut lga = self.find_by_id(id).await?
            .ok_or_else(|| crate::errors::AppError::NotFound("LGA not found".to_string()))?;

        if let Some(name) = &request.name {
            lga.update_name(name.clone());
        }

        if let Some(code) = &request.code {
            let lga_code = LgaCode::new(code.clone())
                .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            lga.update_code(lga_code);
        }

        sqlx::query(
            "UPDATE lgas SET name = $1, code = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(&lga.name)
        .bind(lga.code.to_string())
        .bind(&lga.updated_at)
        .bind(&lga.id)
        .execute(&self.pool)
        .await?;

        Ok(lga)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM lgas WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound("LGA not found".to_string()));
        }

        Ok(())
    }

    async fn count_by_state(&self, state_id: Uuid) -> AppResult<u64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM lgas WHERE state_id = $1")
            .bind(&state_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count") as u64)
    }

    async fn search(&self, query: &str, page: u32, limit: u32) -> AppResult<Vec<Lga>> {
        let offset = (page - 1) * limit;
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            "SELECT id, state_id, name, code, created_at, updated_at FROM lgas WHERE name ILIKE $1 OR code ILIKE $1 ORDER BY name LIMIT $2 OFFSET $3"
        )
        .bind(&search_pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut lgas = Vec::new();
        for row in rows {
            let lga_code = LgaCode::new(row.get::<String, _>("code"))
                .map_err(|e: LgaCodeError| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;
            
            lgas.push(Lga {
                id: row.get("id"),
                state_id: row.get("state_id"),
                name: row.get("name"),
                code: lga_code,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(lgas)
    }
}